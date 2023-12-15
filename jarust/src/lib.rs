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
mod transport;
mod utils;

pub async fn connect(jaconfig: JaConfig) -> JaResult<JaConnection> {
    log::info!("Creating new connection");
    log::trace!("Creating connection with server configuration {jaconfig:?}");

    JaConnection::open(jaconfig).await
}
