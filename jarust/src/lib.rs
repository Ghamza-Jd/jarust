use crate::transport::trans::Transport;
use jaconfig::JaConfig;
use jaconnection::JaConnection;
use prelude::JaResult;

pub mod jaconfig;
pub mod japrotocol;

mod demux;
mod error;
mod jaconnection;
mod jahandle;
mod jasession;
mod prelude;
mod tmanager;
mod transport;
mod utils;

pub async fn connect(jaconfig: JaConfig) -> JaResult<JaConnection> {
    log::info!("Creating new connection");
    log::trace!("Creating connection with server configuration {jaconfig:?}");

    let transport = match jaconfig.transport_type {
        jaconfig::TransportType::Wss => transport::wss::WebsocketTransport::new(),
    };

    JaConnection::open(jaconfig, transport).await
}

pub async fn connect_with_transport(
    jaconfig: JaConfig,
    transport: impl Transport,
) -> JaResult<JaConnection> {
    log::info!("Creating new connection");
    log::trace!("Creating connection with server configuration {jaconfig:?}");

    JaConnection::open(jaconfig, transport).await
}
