use crate::jasession::JaSession;
use crate::jasession::NewSessionParams;
use jarust_interface::janus_interface::JanusInterface;
use jarust_interface::janus_interface::JanusInterfaceImpl;
use jarust_interface::japrotocol::ServerInfoRsp;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct JaConnection {
    interface: JanusInterfaceImpl,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CreateConnectionParams {
    /// Keep alive interval in seconds
    pub ka_interval: u32,
    /// Request timeout
    pub timeout: Duration,
}

impl JaConnection {
    pub(crate) async fn open(
        interface: impl JanusInterface,
    ) -> Result<Self, jarust_interface::Error> {
        tracing::info!("Creating new connection");
        Ok(Self {
            interface: JanusInterfaceImpl::new(interface),
        })
    }

    /// Creates a new session with janus server.
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn create_session(
        &mut self,
        params: CreateConnectionParams,
    ) -> Result<JaSession, jarust_interface::Error> {
        tracing::info!("Creating new session");
        let session_id = self.interface.create(params.timeout).await?;
        let session = JaSession::new(NewSessionParams {
            session_id,
            ka_interval: params.ka_interval,
            interface: self.interface.clone(),
        })
        .await;
        tracing::info!(id = session_id, "Session created");
        Ok(session)
    }

    /// Retrieve Janus server info
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn server_info(
        &mut self,
        timeout: Duration,
    ) -> Result<ServerInfoRsp, jarust_interface::Error> {
        let res = self.interface.server_info(timeout).await?;
        Ok(res)
    }
}
