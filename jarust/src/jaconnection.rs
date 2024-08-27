use crate::jaconfig::JaConfig;
use crate::jasession::JaSession;
use crate::prelude::*;
use jarust_transport::japrotocol::JaResponse;
use jarust_transport::japrotocol::JaSuccessProtocol;
use jarust_transport::japrotocol::ResponseType;
use jarust_transport::jatransport::ConnectionParams;
use jarust_transport::jatransport::JaTransport;
use jarust_transport::legacy::trans::TransportProtocol;
use jarust_transport::respones::ServerInfoRsp;
use jarust_transport::transaction_gen::GenerateTransaction;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

pub type JaResponseStream = mpsc::UnboundedReceiver<JaResponse>;

#[derive(Debug)]
struct InnerConnection {
    transport: JaTransport,
}

#[derive(Clone, Debug)]
pub struct JaConnection {
    inner: Arc<InnerConnection>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CreateConnectionParams {
    /// Keep alive interval in seconds
    pub ka_interval: u32,
    /// Circular buffer capacity
    pub capacity: usize,
    /// Request timeout
    pub timeout: Duration,
}

impl JaConnection {
    pub(crate) async fn open(
        config: JaConfig,
        transport: impl TransportProtocol,
        transaction_generator: impl GenerateTransaction,
    ) -> JaResult<Self> {
        let transport = JaTransport::new(
            ConnectionParams {
                url: config.url,
                capacity: config.capacity,
                apisecret: config.apisecret,
                namespace: config.namespace,
            },
            transport,
            transaction_generator,
        )
        .await?;

        let connection = Arc::new(InnerConnection { transport });
        Ok(Self { inner: connection })
    }

    /// Creates a new session with janus server.
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    pub async fn create(&mut self, params: CreateConnectionParams) -> JaResult<JaSession> {
        tracing::info!("Creating new session");
        let response = self.inner.transport.create(params.timeout).await?;
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

        let session =
            JaSession::new(session_id, params.ka_interval, self.inner.transport.clone()).await;

        tracing::info!("Session created {{ session_id: {session_id} }}");

        Ok(session)
    }
}

impl JaConnection {
    /// Returns janus server info
    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
    pub async fn server_info(&mut self, timeout: Duration) -> JaResult<ServerInfoRsp> {
        let res = self.inner.transport.server_info(timeout).await?;
        Ok(res)
    }
}
