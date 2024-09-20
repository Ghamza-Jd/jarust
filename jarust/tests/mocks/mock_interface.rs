use async_trait::async_trait;
use jarust::prelude::JaResponse;
use jarust::GenerateTransaction;
use jarust_transport::error::JaTransportError;
use jarust_transport::handle_msg::HandleMessage;
use jarust_transport::handle_msg::HandleMessageWithEstablishment;
use jarust_transport::handle_msg::HandleMessageWithEstablishmentAndTimeout;
use jarust_transport::handle_msg::HandleMessageWithTimeout;
use jarust_transport::janus_interface::ConnectionParams;
use jarust_transport::janus_interface::JanusInterface;
use jarust_transport::japrotocol::JaSuccessProtocol;
use jarust_transport::japrotocol::ResponseType;
use jarust_transport::prelude::JaTransportResult;
use jarust_transport::respones::ServerInfoRsp;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub struct Exclusive {
    create_rsp: Option<JaResponse>,
    attach_rsp: Option<JaResponse>,
    handles_rx: HashMap<u64, UnboundedSender<JaResponse>>,
}

#[derive(Debug, Default)]
pub struct InnerMockInterface {
    exclusive: Mutex<Exclusive>,
}

#[derive(Debug, Default, Clone)]
pub struct MockInterface {
    inner: Arc<InnerMockInterface>,
}

#[allow(dead_code)]
impl MockInterface {
    pub async fn mock_create_rsp(&self, rsp: JaResponse) {
        self.inner.exclusive.lock().await.create_rsp = Some(rsp);
    }

    pub async fn mock_attach_rsp(&self, rsp: JaResponse) {
        self.inner.exclusive.lock().await.attach_rsp = Some(rsp);
    }

    pub async fn mock_event(&self, handle_id: u64, rsp: JaResponse) {
        if let Some(tx) = self.inner.exclusive.lock().await.handles_rx.get(&handle_id) {
            tx.send(rsp).unwrap();
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
        let exclusive = Mutex::new(Exclusive::default());
        let inner = InnerMockInterface { exclusive };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    async fn create(&self, _timeout: Duration) -> JaTransportResult<u64> {
        let Some(rsp) = self.inner.exclusive.lock().await.create_rsp.clone() else {
            panic!("Create response is not set");
        };
        let session_id = match rsp.janus {
            ResponseType::Success(JaSuccessProtocol::Data { data }) => data.id,
            ResponseType::Error { error } => {
                let what = JaTransportError::JanusError {
                    code: error.code,
                    reason: error.reason,
                };
                tracing::error!("{what}");
                return Err(what);
            }
            _ => {
                tracing::error!("Unexpected response");
                return Err(JaTransportError::UnexpectedResponse);
            }
        };
        Ok(session_id)
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
        let Some(rsp) = self.inner.exclusive.lock().await.attach_rsp.clone() else {
            panic!("Attach response is not set");
        };
        let handle_id = match rsp.janus {
            ResponseType::Success(JaSuccessProtocol::Data { data }) => data.id,
            ResponseType::Error { error } => {
                let what = JaTransportError::JanusError {
                    code: error.code,
                    reason: error.reason,
                };
                tracing::error!("{what}");
                return Err(what);
            }
            _ => {
                tracing::error!("Unexpected response");
                return Err(JaTransportError::UnexpectedResponse);
            }
        };
        let (tx, rx) = mpsc::unbounded_channel();
        self.inner
            .exclusive
            .lock()
            .await
            .handles_rx
            .insert(handle_id, tx);
        Ok((handle_id, rx))
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
