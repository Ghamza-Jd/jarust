use crate::jaconfig::JaConfig;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::jasession::JaSession;
use crate::napmap::NapMap;
use crate::nw::jatransport::ConnectionParams;
use crate::nw::jatransport::JaTransport;
use crate::nw::transaction_gen::GenerateTransaction;
use crate::params::CreateConnectionParams;
use crate::prelude::*;
use crate::respones::ServerInfoRsp;
use jarust_rt::JaTask;
use jarust_transport::trans::TransportProtocol;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

pub type JaResponseStream = mpsc::UnboundedReceiver<JaResponse>;

#[derive(Debug)]
struct Shared {
    task: JaTask,
    rsp_map: Arc<NapMap<String, JaResponse>>,
    transport: JaTransport,
}

#[derive(Debug)]
struct InnerConnection {
    shared: Shared,
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
        let (transport, mut root_channel) = JaTransport::new(
            ConnectionParams {
                url: &config.url,
                capacity: config.capacity,
                apisecret: config.apisecret,
                namespace: &config.namespace,
            },
            transport,
            transaction_generator,
        )
        .await?;
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

        let shared = Shared {
            task: rsp_cache_task,
            rsp_map,
            transport,
        };
        let connection = Arc::new(InnerConnection { shared });
        Ok(Self { inner: connection })
    }

    /// Creates a new session with janus server.
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    pub async fn create(&mut self, params: CreateConnectionParams) -> JaResult<JaSession> {
        tracing::info!("Creating new session");

        let request = json!({
            "janus": "create"
        });

        let transaction = self.inner.shared.transport.send(request).await?;
        let response = self.poll_response(&transaction, params.timeout).await?;

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

        let channel = self
            .inner
            .shared
            .transport
            .add_session_subroute(session_id)
            .await;

        let session = JaSession::new(
            channel,
            session_id,
            params.ka_interval,
            params.capacity,
            self.inner.shared.transport.clone(),
        )
        .await;

        tracing::info!("Session created {{ session_id: {session_id} }}");

        Ok(session)
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
}

impl JaConnection {
    /// Returns janus server info
    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
    pub async fn server_info(&mut self, timeout: Duration) -> JaResult<ServerInfoRsp> {
        let request = json!({
            "janus": "info"
        });
        let transaction = self.inner.shared.transport.send(request).await?;
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
