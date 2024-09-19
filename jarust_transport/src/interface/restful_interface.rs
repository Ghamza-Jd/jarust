use crate::error::JaTransportError;
use crate::handle_msg::HandleMessage;
use crate::handle_msg::HandleMessageWithEstablishment;
use crate::handle_msg::HandleMessageWithEstablishmentAndTimeout;
use crate::handle_msg::HandleMessageWithTimeout;
use crate::interface::janus_interface::ConnectionParams;
use crate::interface::janus_interface::JanusInterface;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::napmap::NapMap;
use crate::prelude::JaTransportResult;
use crate::respones::ServerInfoRsp;
use crate::router::Router;
use crate::transaction_gen::GenerateTransaction;
use crate::transaction_gen::TransactionGenerator;
use crate::transaction_manager::TransactionManager;
use jarust_rt::JaTask;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

#[derive(Debug)]
struct Shared {
    tasks: Vec<JaTask>,
    namespace: String,
    apisecret: Option<String>,
    transaction_generator: TransactionGenerator,
    rsp_map: Arc<NapMap<String, JaResponse>>,
    client: reqwest::Client,
    baseurl: String,
}

#[derive(Debug)]
struct Exclusive {
    router: Router,
    transaction_manager: TransactionManager,
}

#[derive(Debug)]
struct InnerResultfulInterface {
    shared: Shared,
    exclusive: Mutex<Exclusive>,
}

#[derive(Debug, Clone)]
pub struct RestfulInterface {
    inner: Arc<InnerResultfulInterface>,
}

#[async_trait::async_trait]
impl JanusInterface for RestfulInterface {
    async fn make_interface(
        conn_params: ConnectionParams,
        transaction_generator: impl GenerateTransaction,
    ) -> JaTransportResult<Self> {
        let client = reqwest::Client::new();
        let transaction_generator = TransactionGenerator::new(transaction_generator);
        let transaction_manager = TransactionManager::new(conn_params.capacity);
        let (router, _) = Router::new(&conn_params.namespace).await;
        let shared = Shared {
            tasks: Vec::new(),
            namespace: conn_params.namespace,
            apisecret: conn_params.apisecret,
            transaction_generator,
            rsp_map: Arc::new(NapMap::new(conn_params.capacity)),
            client,
            baseurl: conn_params.url,
        };
        let exclusive = Exclusive {
            router,
            transaction_manager,
        };
        let inner = InnerResultfulInterface {
            shared,
            exclusive: Mutex::new(exclusive),
        };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    async fn create(&self, timeout: Duration) -> JaTransportResult<u64> {
        let response = self
            .inner
            .shared
            .client
            .post(format!("{}/janus", self.inner.shared.baseurl))
            .timeout(timeout)
            .send()
            .await?
            .json::<JaResponse>()
            .await?;

        let session_id = match response.janus {
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
