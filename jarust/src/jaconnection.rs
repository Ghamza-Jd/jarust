use crate::jaconfig::JaConfig;
use crate::japrotocol::JaConnectionRequestProtocol;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaResponseProtocol;
use crate::japrotocol::JaSuccessProtocol;
use crate::jarouter::JaRouter;
use crate::jasession::JaSession;
use crate::jasession::WeakJaSession;
use crate::prelude::*;
use crate::tmanager::TransactionManager;
use crate::transport::trans::Transport;
use crate::transport::trans::TransportProtocol;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::AbortHandle;

#[derive(Debug)]
struct Shared {
    demux_abort_handle: AbortHandle,
    config: JaConfig,
}

#[derive(Debug)]
struct Exclusive {
    router: JaRouter,
    transport_protocol: TransportProtocol,
    receiver: mpsc::Receiver<JaResponse>,
    sessions: HashMap<u64, WeakJaSession>,
    transaction_manager: TransactionManager,
}

#[derive(Debug)]
struct InnerConnection {
    shared: Shared,
    exclusive: Mutex<Exclusive>,
}

#[derive(Clone, Debug)]
pub struct JaConnection {
    inner: Arc<InnerConnection>,
}

impl JaConnection {
    /// Async task to handle demultiplexing of the inbound stream
    async fn demux_task(
        inbound_stream: mpsc::Receiver<String>,
        router: JaRouter,
        transaction_manager: TransactionManager,
    ) -> JaResult<()> {
        let mut stream = inbound_stream;
        while let Some(next) = stream.recv().await {
            let message = serde_json::from_str::<JaResponse>(&next)?;

            // Check if we have a pending transaction and demux to the proper route
            if let Some(pending) = message
                .transaction
                .clone()
                .and_then(|x| transaction_manager.get(&x))
            {
                if pending.path == router.root_path() {
                    router.pub_root(message).await?;
                } else {
                    router.pub_subroute(&pending.path, message).await?;
                }
                transaction_manager.success_close(&pending.id);
                continue;
            }

            // Try get the route from the response
            if let Some(path) = JaRouter::path_from_response(message.clone()) {
                router.pub_subroute(&path, message).await?;
                continue;
            }

            // Fallback to publishing on the root route
            router.pub_root(message).await?;
        }
        Ok(())
    }

    pub(crate) async fn open(config: JaConfig, transport: impl Transport) -> JaResult<Self> {
        let (router, root_channel) = JaRouter::new(&config.namespace).await;
        let transaction_manager = TransactionManager::new();

        let (transport_protocol, receiver) =
            TransportProtocol::connect(transport, &config.uri).await?;

        let demux_join_handle = tokio::spawn({
            let router = router.clone();
            let transaction_manager = transaction_manager.clone();
            async move { JaConnection::demux_task(receiver, router, transaction_manager).await }
        });

        let shared = Shared {
            demux_abort_handle: demux_join_handle.abort_handle(),
            config,
        };
        let safe = Exclusive {
            router,
            transport_protocol,
            receiver: root_channel,
            sessions: HashMap::new(),
            transaction_manager,
        };
        let connection = Arc::new(InnerConnection {
            shared,
            exclusive: Mutex::new(safe),
        });
        Ok(Self { inner: connection })
    }

    /// Creates a new session with janus server.
    pub async fn create(&mut self, ka_interval: u32) -> JaResult<JaSession> {
        log::info!("Creating new session");

        let request = json!({
            "janus": JaConnectionRequestProtocol::CreateSession,
        });

        self.send_request(request).await?;
        let response = match self.inner.exclusive.lock().await.receiver.recv().await {
            Some(response) => response,
            None => {
                log::error!("Incomplete packet");
                return Err(JaError::IncompletePacket);
            }
        };

        let session_id = match response.janus {
            JaResponseProtocol::Success(JaSuccessProtocol::Data { data }) => data.id,
            JaResponseProtocol::Error { error } => {
                let what = JaError::JanusError {
                    code: error.code,
                    reason: error.reason,
                };
                log::error!("{what}");
                return Err(what);
            }
            _ => {
                log::error!("Unexpected response");
                return Err(JaError::UnexpectedResponse);
            }
        };

        let channel = self.add_subroute(&format!("{session_id}")).await;

        let session = JaSession::new(self.clone(), channel, session_id, ka_interval).await;
        self.inner
            .exclusive
            .lock()
            .await
            .sessions
            .insert(session_id, session.downgrade());

        log::info!("Session created {{ id: {session_id} }}");

        Ok(session)
    }

    pub async fn server_info(&mut self) -> JaResult<JaResponse> {
        let request = json!({
            "janus": JaConnectionRequestProtocol::ServerInfo,
        });

        self.send_request(request).await?;
        let response = match self.inner.exclusive.lock().await.receiver.recv().await {
            Some(response) => response,
            None => {
                log::error!("Incomplete packet");
                return Err(JaError::IncompletePacket);
            }
        };
        Ok(response)
    }

    pub(crate) async fn send_request(&mut self, request: Value) -> JaResult<()> {
        let request = self.decorate_request(request);
        let message = serde_json::to_string(&request)?;

        let (Some(janus_request), Some(transaction)) =
            (request["janus"].as_str(), request["transaction"].as_str())
        else {
            let err = JaError::InvalidJanusRequest {
                reason: "request type and/or transaction are missing".to_owned(),
            };
            log::error!("{err}");
            return Err(err);
        };

        let path = JaRouter::path_from_request(&request)
            .unwrap_or(self.inner.shared.config.namespace.clone());

        let mut guard = self.inner.exclusive.lock().await;
        guard
            .transaction_manager
            .create_transaction(transaction, janus_request, &path);
        guard.transport_protocol.send(message.as_bytes()).await
    }

    fn decorate_request(&self, mut request: Value) -> Value {
        let transaction = TransactionManager::random_transaction();
        request["apisecret"] = self.inner.shared.config.apisecret.clone().into();
        request["transaction"] = transaction.into();
        request
    }

    pub(crate) async fn add_subroute(&self, end: &str) -> mpsc::Receiver<JaResponse> {
        self.inner
            .exclusive
            .lock()
            .await
            .router
            .add_subroute(end)
            .await
    }
}

impl Drop for InnerConnection {
    fn drop(&mut self) {
        log::trace!("Connection dropped");
        self.shared.demux_abort_handle.abort();
    }
}
