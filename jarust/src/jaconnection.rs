use crate::demuxer::Demuxer;
use crate::jaconfig::JaConfig;
use crate::japrotocol::JaConnectionRequestProtocol;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::jarouter::JaRouter;
use crate::jasession::JaSession;
use crate::jasession::WeakJaSession;
use crate::jatask;
use crate::prelude::*;
use crate::tmanager::TransactionManager;
use jarust_transport::trans::TransportProtocol;
use jarust_transport::trans::TransportSession;
use jatask::AbortHandle;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub type JaResponseStream = mpsc::UnboundedReceiver<JaResponse>;

#[derive(Debug)]
struct Shared {
    demux_abort_handle: AbortHandle,
    config: JaConfig,
}

#[derive(Debug)]
struct Exclusive {
    router: JaRouter,
    transport_session: TransportSession,
    receiver: JaResponseStream,
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
        let (router, root_channel) = JaRouter::new(&config.namespace).await;
        let transaction_manager = TransactionManager::new(32);

        let (transport_session, receiver) =
            TransportSession::connect(transport, &config.uri).await?;

        let demux_abort_handle = jatask::spawn({
            let router = router.clone();
            let transaction_manager = transaction_manager.clone();
            async move { Demuxer::demux_task(receiver, router, transaction_manager).await }
        });

        let shared = Shared {
            demux_abort_handle,
            config,
        };
        let safe = Exclusive {
            router,
            transport_session,
            receiver: root_channel,
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
    pub async fn create(&mut self, ka_interval: u32) -> JaResult<JaSession> {
        tracing::info!("Creating new session");

        let request = json!({
            "janus": JaConnectionRequestProtocol::CreateSession,
        });

        self.send_request(request).await?;
        let response = match self.inner.exclusive.lock().await.receiver.recv().await {
            Some(response) => response,
            None => {
                tracing::error!("Incomplete packet");
                return Err(JaError::IncompletePacket);
            }
        };

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
                reason: "request type and/or transaction are missing".to_owned(),
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
        tracing::debug!("Sending {message}");
        guard.transport_session.send(message.as_bytes()).await?;
        Ok(transaction.into())
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
    pub async fn server_info(&mut self) -> JaResult<JaResponse> {
        let request = json!({
            "janus": JaConnectionRequestProtocol::ServerInfo,
        });

        self.send_request(request).await?;
        let response = match self.inner.exclusive.lock().await.receiver.recv().await {
            Some(response) => response,
            None => {
                tracing::error!("Incomplete packet");
                return Err(JaError::IncompletePacket);
            }
        };
        Ok(response)
    }
}

impl Drop for InnerConnection {
    #[tracing::instrument(parent = None, level = tracing::Level::TRACE, skip_all)]
    fn drop(&mut self) {
        tracing::debug!("Connection dropped");
        self.shared.demux_abort_handle.abort();
    }
}
