//! # Jarust
//!
//! Jarust is a Rust adapter for [Janus WebRTC server](https://github.com/meetecho/janus-gateway)
//!
//! It provides a high-level API to interact with Janus server.
//!
//! You can use it to connect with Janus server, create a session,
//! attach a plugin, send messages to the plugin, and handle the incoming messages.
//!
//! ## Jarust Transport
//!
//! The details of the connection and the underlying transport are inside the [`jarust_transport`] crate.
//! [`jarust_transport`] provides a customizable interface and transaction generator to customize jarust as per your needs.
//!
//! ### Interfaces
//! jarust_transport provides two default interfaces, [`WebSocketInterface`](jarust_transport::websocket::WebSocketInterface) and [`RestfulInterface`](jarust_transport::restful::RestfulInterface).
//!
//! ### Transaction Generators
//! On the transaction generator side it provides a [`RandomTransactionGenerator`](jarust_transport::tgenerator::RandomTransactionGenerator) and [`UuidTransactionGenerator`](jarust_transport::tgenerator::UuidTransactionGenerator).
//!
//! You could also bring your own transaction generator by implementing the [`GenerateTransaction`](jarust_transport::tgenerator::GenerateTransaction) trait. For example, if you want to use uuid v7.
//!
//! ## Runtime
//!
//! We currently only support tokio runtime and planning to support more runtimes in the future. For that we've abstracted the runtime specific code in the [`jarust_rt`] crate.
//!
//! ## Plugins
//!
//! We have a separate crate for janus plugins, [`jarust_plugins`](https://crates.io/crates/jarust_plugins).
//!
//! For now it supports:
//! - EchoTest plugin
//! - AudioBridge plugin
//! - VideoRoom plugin
//!

pub mod error;
pub mod jaconfig;
pub mod jaconnection;
pub mod jahandle;
pub mod japlugin;
pub mod jasession;
pub mod prelude;

pub use jarust_transport::tgenerator::GenerateTransaction;

use jaconfig::ApiInterface;
use jaconfig::JaConfig;
use jaconnection::JaConnection;
use jarust_transport::janus_interface::ConnectionParams;
use jarust_transport::janus_interface::JanusInterface;
use jarust_transport::restful::RestfulInterface;
use jarust_transport::websocket::WebSocketInterface;
use prelude::JaResult;
use tracing::Level;

/// Creates a new connection with janus server from the provided configs.
///
/// ## Example:
///
/// ```rust
/// use jarust::jaconfig::JaConfig;
/// use jarust::jaconfig::ApiInterface;
/// use jarust_transport::tgenerator::RandomTransactionGenerator;
///
/// let mut connection = jarust::connect(
///     JaConfig::new("ws://localhost:8188/ws", None, "janus"),
///     ApiInterface::WebSocket,
///     RandomTransactionGenerator,
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
        server_root: jaconfig.server_root,
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

/// Creates a new customized connection with janus servers.
#[tracing::instrument(level = Level::TRACE, skip_all)]
pub async fn custom_connect(interface: impl JanusInterface) -> JaResult<JaConnection> {
    JaConnection::open(interface).await
}
