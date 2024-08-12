use super::websocket::WebsocketTransport;
use crate::handle_msg::HandleMessage;
use crate::handle_msg::HandleMessageWithEstablishment;
use crate::handle_msg::HandleMessageWithEstablishmentAndTimeout;
use crate::handle_msg::HandleMessageWithTimeout;
use crate::japrotocol::JaResponse;
use crate::prelude::JaTransportResult;
use crate::respones::ServerInfoRsp;
use crate::transport::ConnectionParams;
use crate::transport::JanusTransport;
use serde::de::DeserializeOwned;
use std::time::Duration;

#[async_trait::async_trait]
impl JanusTransport for WebsocketTransport {
    async fn create_transport(conn_params: ConnectionParams) -> JaTransportResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    async fn create(&self, timeout: Duration) -> JaTransportResult<JaResponse> {
        todo!()
    }

    async fn server_info(&self, timeout: Duration) -> JaTransportResult<ServerInfoRsp> {
        todo!()
    }

    async fn attach(
        &self,
        session_id: u64,
        plugin_id: String,
        timeout: Duration,
    ) -> JaTransportResult<JaResponse> {
        todo!()
    }

    async fn keep_alive(&self, session_id: u64, timeout: Duration) -> JaTransportResult<()> {
        todo!()
    }

    async fn destory(&self, session_id: u64, timeout: Duration) -> JaTransportResult<()> {
        todo!()
    }

    async fn fire_and_forget_msg(&self, message: HandleMessage) -> JaTransportResult<()> {
        todo!()
    }

    async fn send_msg_waiton_ack(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse> {
        todo!()
    }

    async fn send_msg_waiton_rsp<R>(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<R>
    where
        R: DeserializeOwned,
    {
        todo!()
    }

    async fn fire_and_forget_msg_with_establishment(
        &self,
        message: HandleMessageWithEstablishment,
    ) -> JaTransportResult<()> {
        todo!()
    }

    async fn send_msg_waiton_ack_with_establishment(
        &self,
        message: HandleMessageWithEstablishmentAndTimeout,
    ) -> JaTransportResult<JaResponse> {
        todo!()
    }
}
