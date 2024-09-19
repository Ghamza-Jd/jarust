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
use serde::de::DeserializeOwned;
use std::ops::Deref;
use std::time::Duration;
use tokio::sync::mpsc;

#[async_trait::async_trait]
pub trait JanusInterface: 'static {
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
}

pub struct JanusInterfaceImpl {
    inner: Box<dyn JanusInterface>,
}

impl Deref for JanusInterfaceImpl {
    type Target = Box<dyn JanusInterface>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl JanusInterfaceImpl {
    pub fn new(interface: impl JanusInterface) -> Self {
        Self {
            inner: Box::new(interface),
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
