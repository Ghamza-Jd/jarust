use crate::error::JaTransportError;
use crate::handle_msg::HandleMessage;
use crate::handle_msg::HandleMessageWithEstablishment;
use crate::handle_msg::HandleMessageWithEstablishmentAndTimeout;
use crate::handle_msg::HandleMessageWithTimeout;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::prelude::JaTransportResult;
use crate::respones::ServerInfoRsp;
use crate::tgenerator::GenerateTransaction;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

pub struct ConnectionParams {
    /// The url of the janus server.
    pub url: String,
    /// The capacity of the connection (for the websocket interface).
    pub capacity: usize,
    /// The api secret (if any).
    pub apisecret: Option<String>,
    /// The server root, it should match the server root of the janus server when choosing the restful interface.
    pub server_root: String,
}

/// [`JanusInterface`] is the main trait that defines the interface for the janus server.
///
/// It acts as a contract to implement different interfaces supported by janus server,
/// full docs: <https://janus.conf.meetecho.com/docs/rest.html>
#[async_trait::async_trait]
pub trait JanusInterface: Debug + Send + Sync + 'static {
    /// Constructs a new interface with the given connection parameters and transaction generator.
    async fn make_interface(
        conn_params: ConnectionParams,
        transaction_generator: impl GenerateTransaction,
    ) -> JaTransportResult<Self>
    where
        Self: Sized;

    /// Creates a new session with the janus server.
    async fn create(&self, timeout: Duration) -> JaTransportResult<u64>;

    /// Gets the server info.
    async fn server_info(&self, timeout: Duration) -> JaTransportResult<ServerInfoRsp>;

    /// Attaches a plugin to the session.
    async fn attach(
        &self,
        session_id: u64,
        plugin_id: String,
        timeout: Duration,
    ) -> JaTransportResult<(u64, mpsc::UnboundedReceiver<JaResponse>)>;

    /// Send keep alive messages (to keep the connection and the client-server session alive).
    async fn keep_alive(&self, session_id: u64, timeout: Duration) -> JaTransportResult<()>;

    /// Destroys the session.
    async fn destory(&self, session_id: u64, timeout: Duration) -> JaTransportResult<()>;

    /// Sends a one-shot message
    async fn fire_and_forget_msg(&self, message: HandleMessage) -> JaTransportResult<()>;

    /// Sends a message and waits for acknowledgment.
    async fn send_msg_waiton_ack(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse>;

    /// Internal method to send a message and wait for the response. Ideally, this shouldn't be internal,
    /// but we can't have a generic return type for this method as it would be considered object-unsafe.
    ///
    /// Being object-unsafe means we can't use it as a trait object (dyn JanusInterface). Therefore, we have to
    /// make this internal, and the public method that uses this one will have a generic return type.
    /// See [`JanusInterfaceImpl::send_msg_waiton_rsp`] for the public method.
    ///
    /// Check this stack overflow asnwer for the technicalities:
    /// [Why are trait methods with generic type parameters are object unsafe](https://stackoverflow.com/questions/67767207/why-are-trait-methods-with-generic-type-parameters-object-unsafe)
    async fn internal_send_msg_waiton_rsp(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse>;

    /// Sends a one-shot message with establishment.
    async fn fire_and_forget_msg_with_est(
        &self,
        message: HandleMessageWithEstablishment,
    ) -> JaTransportResult<()>;

    /// Sends a message and waits for acknowledgment with establishment.
    async fn send_msg_waiton_ack_with_est(
        &self,
        message: HandleMessageWithEstablishmentAndTimeout,
    ) -> JaTransportResult<JaResponse>;

    /// Returns the name of the interface (for the debug trait)
    fn name(&self) -> Box<str> {
        "Janus Interface".to_string().into_boxed_str()
    }
}

#[derive(Clone)]
pub struct JanusInterfaceImpl {
    inner: Arc<dyn JanusInterface>,
}

impl Deref for JanusInterfaceImpl {
    type Target = Arc<dyn JanusInterface>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl JanusInterfaceImpl {
    pub fn new(interface: impl JanusInterface) -> Self {
        Self {
            inner: Arc::new(interface),
        }
    }

    /// Sends a message and waits for the response.
    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
    pub async fn send_msg_waiton_rsp<R>(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<R>
    where
        R: DeserializeOwned,
    {
        let response = self.internal_send_msg_waiton_rsp(message).await?;
        let result = match response.janus {
            ResponseType::Success(JaSuccessProtocol::Plugin { plugin_data }) => {
                match serde_json::from_value::<R>(plugin_data.data) {
                    Ok(result) => result,
                    Err(error) => {
                        tracing::error!("Failed to parse with error {error:#?}");
                        return Err(JaTransportError::UnexpectedResponse);
                    }
                }
            }
            _ => {
                tracing::error!("Request failed");
                return Err(JaTransportError::UnexpectedResponse);
            }
        };
        Ok(result)
    }
}

impl Debug for JanusInterfaceImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Interface")
            .field(&self.inner.name())
            .finish()
    }
}
