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
use crate::transaction_gen::GenerateTransaction;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

pub struct ConnectionParams {
    pub url: String,
    pub capacity: usize,
    pub apisecret: Option<String>,
    pub namespace: String,
}

#[async_trait::async_trait]
pub trait JanusInterface: Debug + Send + Sync + 'static {
    async fn make_interface(
        conn_params: ConnectionParams,
        transaction_generator: impl GenerateTransaction,
    ) -> JaTransportResult<Self>
    where
        Self: Sized;
    async fn create(&self, timeout: Duration) -> JaTransportResult<u64>;
    async fn server_info(&self, timeout: Duration) -> JaTransportResult<ServerInfoRsp>;
    async fn attach(
        &self,
        session_id: u64,
        plugin_id: String,
        timeout: Duration,
    ) -> JaTransportResult<(u64, mpsc::UnboundedReceiver<JaResponse>)>;
    async fn keep_alive(&self, session_id: u64, timeout: Duration) -> JaTransportResult<()>;
    async fn destory(&self, session_id: u64, timeout: Duration) -> JaTransportResult<()>;
    async fn fire_and_forget_msg(&self, message: HandleMessage) -> JaTransportResult<()>;
    async fn send_msg_waiton_ack(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse>;
    async fn internal_send_msg_waiton_rsp(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse>;
    async fn fire_and_forget_msg_with_est(
        &self,
        message: HandleMessageWithEstablishment,
    ) -> JaTransportResult<()>;
    async fn send_msg_waiton_ack_with_est(
        &self,
        message: HandleMessageWithEstablishmentAndTimeout,
    ) -> JaTransportResult<JaResponse>;

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
