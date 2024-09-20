pub mod error;
pub mod jaconfig;
pub mod jaconnection;
pub mod jahandle;
pub mod japlugin;
pub mod jasession;
pub mod prelude;

pub use jarust_transport::transaction_gen::GenerateTransaction;

use jaconfig::ApiInterface;
use jaconfig::JaConfig;
use jaconnection::JaConnection;
use jarust_transport::interface::janus_interface::ConnectionParams;
use jarust_transport::interface::janus_interface::JanusInterface;
use jarust_transport::interface::restful_interface::RestfulInterface;
use jarust_transport::interface::websocket_interface::WebSocketInterface;
use prelude::JaResult;
use tracing::Level;

/// Creates a new connection with janus server from the provided configs.
///
/// ## Example:
///
/// ```rust
/// let mut connection = jarust::connect(
///     JaConfig::new("ws://localhost:8188/ws", None, "janus"),
///     TransportType::Ws,
///     TransactionGenerationStrategy::Random,
/// )
/// .await
/// .unwrap();
/// ```
#[cfg(not(target_family = "wasm"))]
pub async fn connect(
    jaconfig: JaConfig,
    api_interface: ApiInterface,
    transaction_generator: impl GenerateTransaction,
) -> JaResult<JaConnection> {
    let conn_params = ConnectionParams {
        url: jaconfig.url,
        capacity: jaconfig.capacity,
        apisecret: jaconfig.apisecret,
        namespace: jaconfig.namespace,
    };
    match api_interface {
        ApiInterface::WebSocket => {
            custom_connect(
                WebSocketInterface::make_interface(conn_params, transaction_generator).await?,
            )
            .await
        }
        ApiInterface::Restful => {
            custom_connect(
                RestfulInterface::make_interface(conn_params, transaction_generator).await?,
            )
            .await
        }
    }
}

/// Creates a new connection with janus server from the provided configs
#[cfg(target_family = "wasm")]
pub async fn connect(
    jaconfig: JaConfig,
    api_interface: ApiInterface,
    transaction_generator: impl GenerateTransaction,
) -> JaResult<JaConnection> {
    todo!("WASM is not supported yet")
}

/// Creates a new customized connection with janus server from the provided configs, custom transport, and custom transaction generator.
#[tracing::instrument(level = Level::TRACE)]
pub async fn custom_connect(interface: impl JanusInterface) -> JaResult<JaConnection> {
    tracing::info!("Creating new connection");
    JaConnection::open(interface).await
}
