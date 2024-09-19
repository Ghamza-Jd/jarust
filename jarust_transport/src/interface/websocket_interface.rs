use super::janus_interface::JanusInterface;
use super::websocket_client::WebSocketClient;
use crate::demuxer::Demuxer;
use crate::error::JaTransportError;
use crate::handle_msg::HandleMessage;
use crate::handle_msg::HandleMessageWithEstablishment;
use crate::handle_msg::HandleMessageWithEstablishmentAndTimeout;
use crate::handle_msg::HandleMessageWithTimeout;
use crate::japrotocol::EstablishmentProtocol;
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
use serde_json::json;
use serde_json::Value;
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
    ack_map: Arc<NapMap<String, JaResponse>>,
    rsp_map: Arc<NapMap<String, JaResponse>>,
}

#[derive(Debug)]
struct Exclusive {
    router: Router,
    ws: WebSocketClient,
    transaction_manager: TransactionManager,
}

#[derive(Debug)]
struct InnerWebSocketInterface {
    shared: Shared,
    exclusive: Mutex<Exclusive>,
}

#[derive(Debug, Clone)]
pub struct WebSocketInterface {
    inner: Arc<InnerWebSocketInterface>,
}

pub struct ConnectionParams {
    pub url: String,
    pub capacity: usize,
    pub apisecret: Option<String>,
    pub namespace: String,
}

impl WebSocketInterface {
    pub async fn new(
        conn_params: ConnectionParams,
        transaction_generator: impl GenerateTransaction,
    ) -> JaTransportResult<Self> {
        let (router, _) = Router::new(&conn_params.namespace).await;
        let mut websocket = WebSocketClient::new();
        let receiver = websocket.connect(&conn_params.url).await?;
        let transaction_manager = TransactionManager::new(conn_params.capacity);
        let transaction_generator = TransactionGenerator::new(transaction_generator);

        let ack_map = Arc::new(NapMap::<String, JaResponse>::new(conn_params.capacity));
        let rsp_map = Arc::new(NapMap::<String, JaResponse>::new(conn_params.capacity));

        let (rsp_sender, mut rsp_receiver) = mpsc::unbounded_channel::<JaResponse>();
        let (ack_sender, mut ack_receiver) = mpsc::unbounded_channel::<JaResponse>();

        let rsp_task = jarust_rt::spawn({
            let rsp_map = rsp_map.clone();
            async move {
                while let Some(rsp) = rsp_receiver.recv().await {
                    if let Some(transaction) = rsp.transaction.clone() {
                        rsp_map.insert(transaction, rsp).await;
                    }
                }
            }
        });

        let ack_task = jarust_rt::spawn({
            let ack_map = ack_map.clone();
            async move {
                while let Some(rsp) = ack_receiver.recv().await {
                    if let Some(transaction) = rsp.transaction.clone() {
                        ack_map.insert(transaction, rsp).await;
                    }
                }
            }
        });

        let demux_task = jarust_rt::spawn({
            let router = router.clone();
            let transaction_manager = transaction_manager.clone();
            let demuxer = Demuxer {
                inbound_stream: receiver,
                router,
                rsp_sender,
                ack_sender,
                transaction_manager,
            };
            async move { demuxer.start().await }
        });

        let shared = Shared {
            tasks: vec![demux_task, rsp_task, ack_task],
            namespace: conn_params.namespace,
            apisecret: conn_params.apisecret,
            transaction_generator,
            ack_map,
            rsp_map,
        };
        let exclusive = Exclusive {
            router,
            ws: websocket,
            transaction_manager,
        };
        let inner = InnerWebSocketInterface {
            shared,
            exclusive: Mutex::new(exclusive),
        };
        let this = Self {
            inner: Arc::new(inner),
        };
        Ok(this)
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
    pub async fn send(&self, message: Value) -> JaTransportResult<String> {
        let (message, transaction) = self.decorate_request(message);

        let path =
            Router::path_from_request(&message).unwrap_or(self.inner.shared.namespace.clone());

        let mut guard = self.inner.exclusive.lock().await;
        guard.transaction_manager.insert(&transaction, &path).await;
        guard.ws.send(message.to_string().as_bytes(), &path).await?;
        tracing::debug!("{message:#?}");
        Ok(transaction)
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    async fn poll_response(
        &self,
        transaction: &str,
        timeout: Duration,
    ) -> JaTransportResult<JaResponse> {
        tracing::trace!("Polling response");
        match tokio::time::timeout(
            timeout,
            self.inner.shared.rsp_map.get(transaction.to_string()),
        )
        .await
        {
            Ok(Some(response)) => match response.janus {
                ResponseType::Error { error } => Err(JaTransportError::JanusError {
                    code: error.code,
                    reason: error.reason,
                }),
                _ => Ok(response),
            },
            Ok(None) => {
                tracing::error!("Incomplete packet");
                Err(JaTransportError::IncompletePacket)
            }
            Err(_) => {
                tracing::error!("Request timeout");
                Err(JaTransportError::RequestTimeout)
            }
        }
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    async fn poll_ack(
        &self,
        transaction: &str,
        timeout: Duration,
    ) -> JaTransportResult<JaResponse> {
        tracing::trace!("Polling ack");
        match tokio::time::timeout(
            timeout,
            self.inner.shared.ack_map.get(transaction.to_string()),
        )
        .await
        {
            Ok(Some(response)) => match response.janus {
                ResponseType::Error { error } => Err(JaTransportError::JanusError {
                    code: error.code,
                    reason: error.reason,
                }),
                _ => Ok(response),
            },
            Ok(None) => {
                tracing::error!("Incomplete packet");
                Err(JaTransportError::IncompletePacket)
            }
            Err(_) => {
                tracing::error!("Request timeout");
                Err(JaTransportError::RequestTimeout)
            }
        }
    }

    fn decorate_request(&self, mut request: Value) -> (Value, String) {
        let transaction = self
            .inner
            .shared
            .transaction_generator
            .generate_transaction();
        if let Some(apisecret) = self.inner.shared.apisecret.clone() {
            request["apisecret"] = apisecret.into();
        };
        request["transaction"] = transaction.clone().into();
        (request, transaction)
    }
}

#[async_trait::async_trait]
impl JanusInterface for WebSocketInterface {
    async fn create(&self, timeout: Duration) -> JaTransportResult<u64> {
        let request = json!({
            "janus": "create"
        });

        let transaction = self.send(request).await?;
        let response = self.poll_response(&transaction, timeout).await?;
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
        let request = json!({
            "janus": "info"
        });
        let transaction = self.send(request).await?;
        let response = self.poll_response(&transaction, timeout).await?;
        match response.janus {
            ResponseType::ServerInfo(info) => Ok(*info),
            ResponseType::Error { error } => Err(JaTransportError::JanusError {
                code: error.code,
                reason: error.reason,
            }),
            _ => Err(JaTransportError::IncompletePacket),
        }
    }

    async fn attach(
        &self,
        session_id: u64,
        plugin_id: String,
        timeout: Duration,
    ) -> JaTransportResult<(u64, mpsc::UnboundedReceiver<JaResponse>)> {
        let request = json!({
            "janus": "attach",
            "session_id": session_id,
            "plugin": plugin_id
        });
        let transaction = self.send(request).await?;
        let response = self.poll_response(&transaction, timeout).await?;
        let handle_id = match response.janus {
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
        let receiver = self
            .inner
            .exclusive
            .lock()
            .await
            .router
            .add_subroute(&format!("{session_id}/{handle_id}"))
            .await;
        Ok((handle_id, receiver))
    }

    async fn keep_alive(&self, session_id: u64, timeout: Duration) -> JaTransportResult<()> {
        let request = json!({
            "janus": "keepalive",
            "session_id": session_id
        });
        let transaction = self.send(request).await?;
        self.poll_ack(&transaction, timeout).await?;
        Ok(())
    }

    async fn destory(&self, session_id: u64, timeout: Duration) -> JaTransportResult<()> {
        let request = json!({
            "janus": "destroy",
            "session_id": session_id
        });
        let transaction = self.send(request).await?;
        self.poll_response(&transaction, timeout).await?;
        Ok(())
    }

    async fn fire_and_forget_msg(&self, message: HandleMessage) -> JaTransportResult<()> {
        let request = json!({
            "janus": "message",
            "session_id": message.session_id,
            "handle_id": message.handle_id,
            "body": message.body
        });
        self.send(request).await?;
        Ok(())
    }

    async fn send_msg_waiton_ack(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse> {
        let request = json!({
            "janus": "message",
            "session_id": message.session_id,
            "handle_id": message.handle_id,
            "body": message.body
        });
        let transaction = self.send(request).await?;
        self.poll_ack(&transaction, message.timeout).await
    }

    async fn internal_send_msg_waiton_rsp(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse> {
        let request = json!({
            "janus": "message",
            "session_id": message.session_id,
            "handle_id": message.handle_id,
            "body": message.body
        });
        let transaction = self.send(request).await?;
        self.poll_response(&transaction, message.timeout).await
    }

    async fn fire_and_forget_msg_with_est(
        &self,
        message: HandleMessageWithEstablishment,
    ) -> JaTransportResult<()> {
        let mut request = json!({
            "janus": "message",
            "session_id": message.session_id,
            "handle_id": message.handle_id,
            "body": message.body,
        });
        match message.protocol {
            EstablishmentProtocol::JSEP(jsep) => {
                request["jsep"] = serde_json::to_value(jsep)?;
            }
            EstablishmentProtocol::RTP(rtp) => {
                request["rtp"] = serde_json::to_value(rtp)?;
            }
        };
        self.send(request).await?;
        Ok(())
    }

    async fn send_msg_waiton_ack_with_est(
        &self,
        message: HandleMessageWithEstablishmentAndTimeout,
    ) -> JaTransportResult<JaResponse> {
        let mut request = json!({
            "janus": "message",
            "session_id": message.session_id,
            "handle_id": message.handle_id,
            "body": message.body,
        });
        match message.protocol {
            EstablishmentProtocol::JSEP(jsep) => {
                request["jsep"] = serde_json::to_value(jsep)?;
            }
            EstablishmentProtocol::RTP(rtp) => {
                request["rtp"] = serde_json::to_value(rtp)?;
            }
        };
        let transaction = self.send(request).await?;
        self.poll_ack(&transaction, message.timeout).await
    }
}

impl Drop for InnerWebSocketInterface {
    fn drop(&mut self) {
        self.shared.tasks.iter().for_each(|task| {
            task.cancel();
        });
    }
}
