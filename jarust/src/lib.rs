pub mod error;
pub mod jaconfig;
pub mod jaconnection;
pub mod jahandle;
pub mod japlugin;
pub mod japrotocol;
pub mod jasession;
pub mod prelude;
pub mod transport;

mod jarouter;
mod nsp_registry;
mod tmanager;
mod utils;

use crate::transport::trans::Transport;
use jaconfig::JaConfig;
use jaconfig::TransportType;
use jaconnection::JaConnection;
use prelude::JaResult;

#[cfg(not(target_family = "wasm"))]
/// Creates a new connection with janus server from the provided configs
pub async fn connect(jaconfig: JaConfig, transport_type: TransportType) -> JaResult<JaConnection> {
    let transport = match transport_type {
        jaconfig::TransportType::Wss => transport::wss::WebsocketTransport::new(),
    };
    connect_with_transport(jaconfig, transport).await
}

#[cfg(target_family = "wasm")]
/// Creates a new connection with janus server from the provided configs
pub async fn connect(jaconfig: JaConfig, transport_type: TransportType) -> JaResult<JaConnection> {
    let transport = transport::wasm_wss::WasmWsTransport;
    connect_with_transport(jaconfig, transport).await
}

/// Creates a new connection with janus server from the provided configs and custom transport
pub async fn connect_with_transport(
    jaconfig: JaConfig,
    transport: impl Transport,
) -> JaResult<JaConnection> {
    log::info!("Creating new connection");
    log::trace!("Creating connection with server configuration {jaconfig:?}");
    JaConnection::open(jaconfig, transport).await
}
