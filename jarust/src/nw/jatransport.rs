use super::demuxer::Demuxer;
use super::router::Router;
use super::transaction_gen::TransactionGenerator;
use super::transaction_manager::TransactionManager;
use crate::japrotocol::EstablishmentProtocol;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::napmap::NapMap;
use crate::prelude::JaError;
use crate::prelude::JaResult;
use crate::respones::ServerInfoRsp;
use crate::GenerateTransaction;
use jarust_rt::JaTask;
use jarust_transport::trans::TransportProtocol;
use jarust_transport::trans::TransportSession;
use serde::de::DeserializeOwned;
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
    transport: TransportSession,
    transaction_manager: TransactionManager,
}

#[derive(Debug)]
struct InnerJaTransport {
    shared: Shared,
    exclusive: Mutex<Exclusive>,
}

#[derive(Debug, Clone)]
pub struct JaTransport {
    inner: Arc<InnerJaTransport>,
}

pub struct ConnectionParams<'a> {
    pub url: &'a str,
    pub capacity: usize,
    pub apisecret: Option<String>,
    pub namespace: &'a str,
}

impl JaTransport {
    pub async fn new(
        conn_params: ConnectionParams<'_>,
        transport: impl TransportProtocol,
        transaction_generator: impl GenerateTransaction,
    ) -> JaResult<(Self, mpsc::UnboundedReceiver<JaResponse>)> {
        let (router, root_channel) = Router::new(conn_params.namespace).await;
        let (transport, receiver) = TransportSession::connect(transport, conn_params.url).await?;
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
            namespace: conn_params.namespace.into(),
            apisecret: conn_params.apisecret,
            transaction_generator,
            ack_map,
            rsp_map,
        };
        let exclusive = Exclusive {
            router,
            transport,
            transaction_manager,
        };
        let inner = InnerJaTransport {
            shared,
            exclusive: Mutex::new(exclusive),
        };
        let this = Self {
            inner: Arc::new(inner),
        };
        Ok((this, root_channel))
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
    pub async fn send(&self, message: Value) -> JaResult<String> {
        let (message, transaction) = self.decorate_request(message);

        let path =
            Router::path_from_request(&message).unwrap_or(self.inner.shared.namespace.clone());

        let mut guard = self.inner.exclusive.lock().await;
        guard.transaction_manager.insert(&transaction, &path).await;
        guard
            .transport
            .send(message.to_string().as_bytes(), &path)
            .await?;
        tracing::debug!("{message:#?}");
        Ok(transaction)
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    async fn poll_response(&self, transaction: &str, timeout: Duration) -> JaResult<JaResponse> {
        tracing::trace!("Polling response");
        match tokio::time::timeout(
            timeout,
            self.inner.shared.rsp_map.get(transaction.to_string()),
        )
        .await
        {
            Ok(Some(response)) => match response.janus {
                ResponseType::Error { error } => Err(JaError::JanusError {
                    code: error.code,
                    reason: error.reason,
                }),
                _ => Ok(response),
            },
            Ok(None) => {
                tracing::error!("Incomplete packet");
                Err(JaError::IncompletePacket)
            }
            Err(_) => {
                tracing::error!("Request timeout");
                Err(JaError::RequestTimeout)
            }
        }
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    async fn poll_ack(&self, transaction: &str, timeout: Duration) -> JaResult<JaResponse> {
        tracing::trace!("Polling ack");
        match tokio::time::timeout(
            timeout,
            self.inner.shared.ack_map.get(transaction.to_string()),
        )
        .await
        {
            Ok(Some(response)) => match response.janus {
                ResponseType::Error { error } => Err(JaError::JanusError {
                    code: error.code,
                    reason: error.reason,
                }),
                _ => Ok(response),
            },
            Ok(None) => {
                tracing::error!("Incomplete packet");
                Err(JaError::IncompletePacket)
            }
            Err(_) => {
                tracing::error!("Request timeout");
                Err(JaError::RequestTimeout)
            }
        }
    }

    pub async fn add_session_subroute(
        &self,
        session_id: u64,
    ) -> mpsc::UnboundedReceiver<JaResponse> {
        self.inner
            .exclusive
            .lock()
            .await
            .router
            .add_subroute(&format!("{session_id}"))
            .await
    }

    pub async fn add_handle_subroute(
        &self,
        session_id: u64,
        handle_id: u64,
    ) -> mpsc::UnboundedReceiver<JaResponse> {
        self.inner
            .exclusive
            .lock()
            .await
            .router
            .add_subroute(&format!("{session_id}/{handle_id}"))
            .await
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

impl JaTransport {
    pub async fn create(&self, timeout: Duration) -> JaResult<JaResponse> {
        let request = json!({
            "janus": "create"
        });

        let transaction = self.send(request).await?;
        self.poll_response(&transaction, timeout).await
    }

    pub async fn server_info(&self, timeout: Duration) -> JaResult<ServerInfoRsp> {
        let request = json!({
            "janus": "info"
        });
        let transaction = self.send(request).await?;
        let response = self.poll_response(&transaction, timeout).await?;
        match response.janus {
            ResponseType::ServerInfo(info) => Ok(*info),
            ResponseType::Error { error } => Err(JaError::JanusError {
                code: error.code,
                reason: error.reason,
            }),
            _ => Err(JaError::IncompletePacket),
        }
    }

    pub async fn attach(
        &self,
        session_id: u64,
        plugin_id: String,
        timeout: Duration,
    ) -> JaResult<JaResponse> {
        let request = json!({
            "janus": "attach",
            "session_id": session_id,
            "plugin": plugin_id
        });
        let transaction = self.send(request).await?;
        self.poll_response(&transaction, timeout).await
    }

    pub async fn keep_alive(&self, session_id: u64, timeout: Duration) -> JaResult<()> {
        let request = json!({
            "janus": "keepalive",
            "session_id": session_id
        });
        let transaction = self.send(request).await?;
        self.poll_ack(&transaction, timeout).await?;
        Ok(())
    }

    pub async fn destory(&self, session_id: u64, timeout: Duration) -> JaResult<()> {
        let request = json!({
            "janus": "destroy",
            "session_id": session_id
        });
        let transaction = self.send(request).await?;
        self.poll_response(&transaction, timeout).await?;
        Ok(())
    }
}

impl JaTransport {
    pub async fn fire_and_forget_msg(
        &self,
        session_id: u64,
        handle_id: u64,
        body: Value,
    ) -> JaResult<()> {
        let request = json!({
            "janus": "message",
            "session_id": session_id,
            "handle_id": handle_id,
            "body": body
        });
        self.send(request).await?;
        Ok(())
    }

    pub async fn send_msg_waiton_ack(
        &self,
        session_id: u64,
        handle_id: u64,
        body: Value,
        timeout: Duration,
    ) -> JaResult<JaResponse> {
        let request = json!({
            "janus": "message",
            "session_id": session_id,
            "handle_id": handle_id,
            "body": body
        });
        let transaction = self.send(request).await?;
        self.poll_ack(&transaction, timeout).await
    }

    pub async fn send_msg_waiton_rsp<R>(
        &self,
        session_id: u64,
        handle_id: u64,
        body: Value,
        timeout: Duration,
    ) -> JaResult<R>
    where
        R: DeserializeOwned,
    {
        let request = json!({
            "janus": "message",
            "session_id": session_id,
            "handle_id": handle_id,
            "body": body
        });
        let transaction = self.send(request).await?;
        let response = self.poll_response(&transaction, timeout).await?;

        let result = match response.janus {
            ResponseType::Success(JaSuccessProtocol::Plugin { plugin_data }) => {
                match serde_json::from_value::<R>(plugin_data.data) {
                    Ok(result) => result,
                    Err(error) => {
                        tracing::error!("Failed to parse with error {error:#?}");
                        return Err(JaError::UnexpectedResponse);
                    }
                }
            }
            _ => {
                tracing::error!("Request failed");
                return Err(JaError::UnexpectedResponse);
            }
        };
        Ok(result)
    }

    pub async fn fire_and_forget_msg_with_establishment(
        &self,
        session_id: u64,
        handle_id: u64,
        body: Value,
        protocol: EstablishmentProtocol,
    ) -> JaResult<()> {
        let request = match protocol {
            EstablishmentProtocol::JSEP(jsep) => json!({
                "janus": "message",
                "session_id": session_id,
                "handle_id": handle_id,
                "body": body,
                "jsep": jsep
            }),
            EstablishmentProtocol::RTP(rtp) => json!({
                "janus": "message",
                "session_id": session_id,
                "handle_id": handle_id,
                "body": body,
                "rtp": rtp
            }),
        };
        self.send(request).await?;
        Ok(())
    }

    pub async fn send_msg_waiton_ack_with_establishment(
        &self,
        session_id: u64,
        handle_id: u64,
        body: Value,
        protocol: EstablishmentProtocol,
        timeout: Duration,
    ) -> JaResult<JaResponse> {
        let request = match protocol {
            EstablishmentProtocol::JSEP(jsep) => json!({
                "janus": "message",
                "session_id": session_id,
                "handle_id": handle_id,
                "body": body,
                "jsep": jsep
            }),
            EstablishmentProtocol::RTP(rtp) => json!({
                "janus": "message",
                "session_id": session_id,
                "handle_id": handle_id,
                "body": body,
                "rtp": rtp
            }),
        };
        let transaction = self.send(request).await?;
        self.poll_ack(&transaction, timeout).await
    }
}

impl Drop for InnerJaTransport {
    fn drop(&mut self) {
        self.shared.tasks.iter().for_each(|task| {
            task.cancel();
        });
    }
}
