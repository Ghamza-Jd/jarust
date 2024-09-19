use super::mock_generate_transaction::MockGenerateTransaction;
use async_trait::async_trait;
use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use jarust::error::JaError;
use jarust::prelude::JaResponse;
use jarust::prelude::JaResult;
use jarust::GenerateTransaction;
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
    server: Option<MockServer>,
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
        _: ConnectionParams,
        _: impl GenerateTransaction,
    ) -> JaTransportResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            server: Some(MockServer {
                tx: mpsc::unbounded_channel().0,
            }),
        })
    }

    async fn create(&self, _timeout: Duration) -> JaTransportResult<u64> {
        todo!("Create is not implemented");
    }

    async fn server_info(&self, _timeout: Duration) -> JaTransportResult<ServerInfoRsp> {
        todo!("Server info is not implemented");
    }

    async fn attach(
        &self,
        _session_id: u64,
        _plugin_id: String,
        _timeout: Duration,
    ) -> JaTransportResult<(u64, mpsc::UnboundedReceiver<JaResponse>)> {
        todo!("Attach is not implemented");
    }

    async fn keep_alive(&self, _session_id: u64, _timeout: Duration) -> JaTransportResult<()> {
        todo!("Keep alive is not implemented");
    }

    async fn destory(&self, _session_id: u64, _timeout: Duration) -> JaTransportResult<()> {
        todo!("Destroy is not implemented");
    }

    async fn fire_and_forget_msg(&self, _message: HandleMessage) -> JaTransportResult<()> {
        todo!("Fire and forget is not implemented");
    }

    async fn send_msg_waiton_ack(
        &self,
        _message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse> {
        todo!("Send message wait on ack is not implemented");
    }

    async fn internal_send_msg_waiton_rsp(
        &self,
        _message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse> {
        todo!("Internal send message wait on response is not implemented");
    }

    async fn fire_and_forget_msg_with_est(
        &self,
        _message: HandleMessageWithEstablishment,
    ) -> JaTransportResult<()> {
        todo!("Fire and forget with establishment is not implemented");
    }

    async fn send_msg_waiton_ack_with_est(
        &self,
        _message: HandleMessageWithEstablishmentAndTimeout,
    ) -> JaTransportResult<JaResponse> {
        todo!("Send message wait on ack with establishment is not implemented");
    }
}
