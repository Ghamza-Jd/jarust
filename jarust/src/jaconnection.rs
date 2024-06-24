use crate::jaconfig::JaConfig;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::jasession::JaSession;
use crate::jasession::WeakJaSession;
use crate::napmap::NapMap;
use crate::nw::nwconn::NetworkConnection;
use crate::nw::nwconn::NwConn;
use crate::nw::transaction_gen::GenerateTransaction;
use crate::nw::transaction_gen::TransactionGenerator;
use crate::prelude::*;
use crate::respones::ServerInfoRsp;
use jarust_rt::JaTask;
use jarust_transport::trans::TransportProtocol;
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
    task: JaTask,
    config: JaConfig,
    rsp_map: Arc<NapMap<String, JaResponse>>,
    transaction_generator: TransactionGenerator,
}

#[derive(Debug)]
struct Exclusive {
    nwconn: NwConn,
    sessions: HashMap<u64, WeakJaSession>,
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
        transaction_generator: impl GenerateTransaction,
    ) -> JaResult<Self> {
        let (nwconn, mut root_channel) =
            NwConn::new(&config.url, &config.namespace, config.capacity, transport).await?;
        let rsp_map = Arc::new(NapMap::new(config.capacity));

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

        let transaction_generator = TransactionGenerator::new(transaction_generator);

        let shared = Shared {
            task: rsp_cache_task,
            config,
            rsp_map,
            transaction_generator,
        };
        let safe = Exclusive {
            nwconn,
            sessions: HashMap::new(),
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
        let transaction = self
            .inner
            .exclusive
            .lock()
            .await
            .nwconn
            .send(request)
            .await?;
        Ok(transaction)
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
        let transaction = self
            .inner
            .shared
            .transaction_generator
            .generate_transaction();
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
            .nwconn
            .add_subroute(end)
            .await
    }
}

impl JaConnection {
    /// Returns janus server info
    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
    pub async fn server_info(&mut self, timeout: Duration) -> JaResult<ServerInfoRsp> {
        let request = json!({
            "janus": "info"
        });
        let transaction = self.send_request(request).await?;
        let response = self.poll_response(&transaction, timeout).await?;
        match response.janus {
            ResponseType::ServerInfo(info) => Ok(*info),
            ResponseType::Error { error } => Err(JaError::JanusError {
                code: error.code,
                reason: error.reason,
            }),
            _ => Err(JaError::IncompletePacket),
        }
    }
}

impl Drop for InnerConnection {
    #[tracing::instrument(parent = None, level = tracing::Level::TRACE, skip_all)]
    fn drop(&mut self) {
        tracing::debug!("JaConnection dropped, cancelling all associated tasks");
        self.shared.task.cancel();
    }
}
