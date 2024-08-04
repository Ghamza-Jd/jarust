use crate::handle_msg::HandleMessage;
use crate::handle_msg::HandleMessageWithEstablishment;
use crate::handle_msg::HandleMessageWithEstablishmentAndTimeout;
use crate::handle_msg::HandleMessageWithTimeout;
use crate::japrotocol::JaResponse;
use crate::prelude::JaTransportResult;
use crate::respones::ServerInfoRsp;
use serde::de::DeserializeOwned;
use std::time::Duration;

#[async_trait::async_trait]
pub trait JanusTransport {
    async fn create(&self, timeout: Duration) -> JaTransportResult<JaResponse>;
    async fn server_info(&self, timeout: Duration) -> JaTransportResult<ServerInfoRsp>;
    async fn attach(
        &self,
        session_id: u64,
        plugin_id: String,
        timeout: Duration,
    ) -> JaTransportResult<JaResponse>;
    async fn keep_alive(&self, session_id: u64, timeout: Duration) -> JaTransportResult<()>;
    async fn destory(&self, session_id: u64, timeout: Duration) -> JaTransportResult<()>;

    async fn fire_and_forget_msg(&self, message: HandleMessage) -> JaTransportResult<()>;
    async fn send_msg_waiton_ack(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse>;
    async fn send_msg_waiton_rsp<R>(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<R>
    where
        R: DeserializeOwned;
    async fn fire_and_forget_msg_with_establishment(
        &self,
        message: HandleMessageWithEstablishment,
    ) -> JaTransportResult<()>;
    async fn send_msg_waiton_ack_with_establishment(
        &self,
        message: HandleMessageWithEstablishmentAndTimeout,
    ) -> JaTransportResult<JaResponse>;
}
