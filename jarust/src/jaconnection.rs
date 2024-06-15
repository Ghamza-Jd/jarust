use crate::demuxer::Demuxer;
use crate::jaconfig::JaConfig;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::jarouter::JaRouter;
use crate::jasession::JaSession;
use crate::jasession::WeakJaSession;
use crate::prelude::*;
use crate::tmanager::TransactionManager;
use jarust_rt::JaTask;
use jarust_transport::trans::TransportProtocol;
use jarust_transport::trans::TransportSession;
use napmap::UnboundedNapMap;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub type JaResponseStream = mpsc::UnboundedReceiver<JaResponse>;

#[derive(Debug)]
struct Shared {
    tasks: Vec<JaTask>,
    config: JaConfig,
    rsp_map: Arc<UnboundedNapMap<String, JaResponse>>,
}

#[derive(Debug)]
struct Exclusive {
    router: JaRouter,
    transport_session: TransportSession,
    sessions: HashMap<u64, WeakJaSession>,
    transaction_manager: TransactionManager,
}

#[derive(Debug)]
struct InnerConnection {
    shared: Shared,
    exclusive: Mutex<Exclusive>,
}

#[derive(Clone, Debug)]
pub struct JaConnection {
    inner: Arc<InnerConnection>,
}

impl JaConnection {
    pub(crate) async fn open(
        config: JaConfig,
        transport: impl TransportProtocol,
    ) -> JaResult<Self> {
        let (router, mut root_channel) = JaRouter::new(&config.namespace).await;
        let rsp_map = Arc::new(napmap::unbounded());
        let transaction_manager = TransactionManager::new(32);

        let (transport_session, receiver) =
            TransportSession::connect(transport, &config.url).await?;

        let demux_task = jarust_rt::spawn({
            let router = router.clone();
            let transaction_manager = transaction_manager.clone();
            async move { Demuxer::demux_task(receiver, router, transaction_manager).await }
        });

        let rsp_cache_task = jarust_rt::spawn({
            let rsp_map = rsp_map.clone();
            async move {
                while let Some(rsp) = root_channel.recv().await {
                    if let Some(transaction) = rsp.transaction.clone() {
                        rsp_map.insert(transaction, rsp).await;
                    }
                }
            }
        });

        let tasks = vec![demux_task, rsp_cache_task];

        let shared = Shared {
            tasks,
            config,
            rsp_map,
        };
        let safe = Exclusive {
            router,
            transport_session,
            sessions: HashMap::new(),
            transaction_manager,
        };
        let connection = Arc::new(InnerConnection {
            shared,
            exclusive: Mutex::new(safe),
        });
        Ok(Self { inner: connection })
    }

    /// Creates a new session with janus server.
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    pub async fn create(&mut self, ka_interval: u32, timeout: Duration) -> JaResult<JaSession> {
        tracing::info!("Creating new session");

        let request = json!({
            "janus": "create"
        });

        let transaction = self.send_request(request).await?;
        let response = self.poll_response(&transaction, timeout).await?;

        let session_id = match response.janus {
            ResponseType::Success(JaSuccessProtocol::Data { data }) => data.id,
            ResponseType::Error { error } => {
                let what = JaError::JanusError {
                    code: error.code,
                    reason: error.reason,
                };
                tracing::error!("{what}");
                return Err(what);
            }
            _ => {
                tracing::error!("Unexpected response");
                return Err(JaError::UnexpectedResponse);
            }
        };

        let channel = self.add_subroute(&format!("{session_id}")).await;

        let session = JaSession::new(self.clone(), channel, session_id, ka_interval).await;
        self.inner
            .exclusive
            .lock()
            .await
            .sessions
            .insert(session_id, session.downgrade());

        tracing::info!("Session created {{ session_id: {session_id} }}");

        Ok(session)
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
    pub(crate) async fn send_request(&mut self, request: Value) -> JaResult<String> {
        let request = self.decorate_request(request);
        let message = serde_json::to_string(&request)?;

        let Some(transaction) = request["transaction"].as_str() else {
            let err = JaError::InvalidJanusRequest {
                reason: "request transaction is missing".to_owned(),
            };
            tracing::error!("{err}");
            return Err(err);
        };

        let path = JaRouter::path_from_request(&request)
            .unwrap_or(self.inner.shared.config.namespace.clone());

        let mut guard = self.inner.exclusive.lock().await;
        guard
            .transaction_manager
            .create_transaction(transaction, &path)
            .await;
        tracing::debug!("Sending {request:#}");
        guard
            .transport_session
            .send(message.as_bytes(), &path)
            .await?;
        Ok(transaction.into())
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
    async fn poll_response(&self, transaction: &str, timeout: Duration) -> JaResult<JaResponse> {
        tracing::trace!("Polling response");
        match tokio::time::timeout(
            timeout,
            self.inner.shared.rsp_map.get(transaction.to_string()),
        )
        .await
        {
            Ok(Some(response)) => match response.janus {
                ResponseType::Error { error } => Err(JaError::JanusError {
                    code: error.code,
                    reason: error.reason,
                }),
                _ => Ok(response),
            },
            Ok(None) => {
                tracing::error!("Incomplete packet");
                Err(JaError::IncompletePacket)
            }
            Err(_) => {
                tracing::error!("Request timeout");
                Err(JaError::RequestTimeout)
            }
        }
    }

    fn decorate_request(&self, mut request: Value) -> Value {
        let transaction = TransactionManager::random_transaction();
        if let Some(apisecret) = self.inner.shared.config.apisecret.clone() {
            request["apisecret"] = apisecret.into();
        };
        request["transaction"] = transaction.into();
        request
    }

    pub(crate) async fn add_subroute(&self, end: &str) -> JaResponseStream {
        self.inner
            .exclusive
            .lock()
            .await
            .router
            .add_subroute(end)
            .await
    }
}

impl JaConnection {
    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
    pub async fn server_info(&mut self, timeout: Duration) -> JaResult<JaResponse> {
        let request = json!({
            "janus": "info"
        });
        let transaction = self.send_request(request).await?;
        let response = self.poll_response(&transaction, timeout).await?;
        Ok(response)
    }
}

impl Drop for InnerConnection {
    #[tracing::instrument(parent = None, level = tracing::Level::TRACE, skip_all)]
    fn drop(&mut self) {
        tracing::debug!("JaConnection dropped, cancelling all associated tasks");
        self.shared.tasks.iter().for_each(|task| {
            task.cancel();
        });
    }
}
