pub mod error;
pub mod jaconfig;
pub mod jaconnection;
pub mod jahandle;
pub mod japlugin;
pub mod jasession;
pub mod nw;
pub mod params;
pub mod prelude;

pub use nw::transaction_gen::GenerateTransaction;
pub use nw::transaction_gen::TransactionGenerationStrategy;

use jaconfig::JaConfig;
use jaconfig::TransportType;
use jaconnection::JaConnection;
use jarust_transport::trans::TransportProtocol;
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
    transport_type: TransportType,
    transaction_generation_strategy: TransactionGenerationStrategy,
) -> JaResult<JaConnection> {
    let transport = match transport_type {
        jaconfig::TransportType::Ws => {
            jarust_transport::web_socket::WebsocketTransport::create_transport()
        }
    };
    custom_connect(
        jaconfig,
        transport,
        transaction_generation_strategy.generator(),
    )
    .await
}

/// Creates a new connection with janus server from the provided configs
#[cfg(target_family = "wasm")]
pub async fn connect(
    jaconfig: JaConfig,
    transport_type: TransportType,
    transaction_generation_strategy: TransactionGenerationStrategy,
) -> JaResult<JaConnection> {
    let transport = transport::wasm_web_socket::WasmWsTransport;
    custom_connect(
        jaconfig,
        transport,
        transaction_generation_strategy.generator(),
    )
    .await
}

/// Creates a new customized connection with janus server from the provided configs, custom transport, and custom transaction generator.
#[tracing::instrument(level = Level::TRACE)]
pub async fn custom_connect(
    jaconfig: JaConfig,
    transport: impl TransportProtocol,
    transaction_generator: impl GenerateTransaction,
) -> JaResult<JaConnection> {
    tracing::info!("Creating new connection");
    JaConnection::open(jaconfig, transport, transaction_generator).await
}
