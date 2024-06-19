pub mod error;
pub mod jaconfig;
pub mod jaconnection;
pub mod jahandle;
pub mod japlugin;
pub mod japrotocol;
pub mod jasession;
pub mod prelude;
pub mod transaction_gen;

mod nw;

use jaconfig::JaConfig;
use jaconfig::TransportType;
use jaconnection::JaConnection;
use jarust_transport::trans::TransportProtocol;
use prelude::JaResult;
use tracing::Level;
use transaction_gen::GenerateTransaction;
use transaction_gen::TransactionGenerationStrategy;

/// Creates a new connection with janus server from the provided configs
///
/// ## Example:
///
/// ```rust
/// let mut connection = jarust::connect(
///     JaConfig::new("ws://localhost:8188/ws", None, "janus"),
///     TransportType::Ws,
/// )
/// .await
/// .unwrap();
/// ```
#[cfg(not(target_family = "wasm"))]
pub async fn connect(
    jaconfig: JaConfig,
    transport_type: TransportType,
    transaction_generation_strategy: TransactionGenerationStrategy,
) -> JaResult<JaConnection> {
    let transport = match transport_type {
        jaconfig::TransportType::Ws => {
            jarust_transport::web_socket::WebsocketTransport::create_transport()
        }
    };
    connect_with_transport(
        jaconfig,
        transport,
        transaction_generation_strategy.generator(),
    )
    .await
}

#[cfg(target_family = "wasm")]
/// Creates a new connection with janus server from the provided configs
pub async fn connect(jaconfig: JaConfig, transport_type: TransportType) -> JaResult<JaConnection> {
    let transport = transport::wasm_web_socket::WasmWsTransport;
    connect_with_transport(jaconfig, transport).await
}

/// Creates a new connection with janus server from the provided configs and custom transport.
///
/// Same as [`connect`], but takes a struct that implements [`Transport`] to be used instead
/// of using one of the predefined transport types [`TransportType`]
#[tracing::instrument(level = Level::TRACE)]
pub async fn connect_with_transport(
    jaconfig: JaConfig,
    transport: impl TransportProtocol,
    transaction_generator: impl GenerateTransaction,
) -> JaResult<JaConnection> {
    tracing::info!("Creating new connection");
    JaConnection::open(jaconfig, transport, transaction_generator).await
}
