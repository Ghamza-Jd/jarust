use super::mock_generate_transaction::MockGenerateTransaction;
use async_trait::async_trait;
use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use jarust::error::JaError;
use jarust::prelude::JaResponse;
use jarust::prelude::JaResult;
use jarust::GenerateTransaction;
use jarust_rt::JaTask;
use jarust_transport::error::JaTransportError;
use jarust_transport::handle_msg::HandleMessage;
use jarust_transport::handle_msg::HandleMessageWithEstablishment;
use jarust_transport::handle_msg::HandleMessageWithEstablishmentAndTimeout;
use jarust_transport::handle_msg::HandleMessageWithTimeout;
use jarust_transport::interface::janus_interface::ConnectionParams;
use jarust_transport::interface::janus_interface::JanusInterface;
use jarust_transport::prelude::JaTransportResult;
use jarust_transport::respones::ServerInfoRsp;
use std::fmt::Debug;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct MockServer {
    tx: mpsc::UnboundedSender<Bytes>,
}

impl MockServer {
    pub async fn mock_send_to_client(&self, msg: &str) {
        let mut bytes = BytesMut::new();
        bytes.put_slice(msg.as_bytes());
        self.tx.send(bytes.into()).unwrap();
    }
}

#[derive(Debug)]
pub struct MockInterface {
    rx: Option<mpsc::UnboundedReceiver<Bytes>>,
    server: Option<MockServer>,
    task: Option<JaTask>,
}

impl MockInterface {
    pub fn get_mock_server(&mut self) -> Option<MockServer> {
        self.server.take()
    }

    pub async fn interface_server_pair() -> JaResult<(Self, MockServer)> {
        let conn_params = ConnectionParams {
            url: "mock://some.janus.com".to_string(),
            capacity: 10,
            apisecret: None,
            namespace: "mock".to_string(),
        };
        let transaction_generator = MockGenerateTransaction::new();
        let mut interface =
            MockInterface::make_interface(conn_params, transaction_generator).await?;
        match interface.get_mock_server() {
            Some(server) => Ok((interface, server)),
            None => Err(JaError::JanusTransport(
                JaTransportError::TransportNotOpened,
            )),
        }
    }
}

#[async_trait]
impl JanusInterface for MockInterface {
    async fn make_interface(
        conn_params: ConnectionParams,
        transaction_generator: impl GenerateTransaction,
    ) -> JaTransportResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    async fn create(&self, timeout: Duration) -> JaTransportResult<u64> {
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
    ) -> JaTransportResult<(u64, mpsc::UnboundedReceiver<JaResponse>)> {
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

    async fn internal_send_msg_waiton_rsp(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse> {
        todo!()
    }

    async fn fire_and_forget_msg_with_est(
        &self,
        message: HandleMessageWithEstablishment,
    ) -> JaTransportResult<()> {
        todo!()
    }

    async fn send_msg_waiton_ack_with_est(
        &self,
        message: HandleMessageWithEstablishmentAndTimeout,
    ) -> JaTransportResult<JaResponse> {
        todo!()
    }
}
