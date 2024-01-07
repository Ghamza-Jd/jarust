use crate::jaconfig::JaConfig;
use crate::japrotocol::JaConnectionRequestProtocol;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaResponseProtocol;
use crate::japrotocol::JaSuccessProtocol;
use crate::jasession::JaSession;
use crate::jasession::WeakJaSession;
use crate::nsp_registry::NamespaceRegistry;
use crate::prelude::*;
use crate::tmanager::TransactionManager;
use crate::transport::trans::Transport;
use crate::transport::trans::TransportProtocol;
use crate::utils::generate_transaction;
use crate::utils::get_subnamespace_from_request;
use crate::utils::get_subnamespace_from_response;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::AbortHandle;

struct Shared {
    demux_abort_handle: AbortHandle,
    config: JaConfig,
}

struct SafeShared {
    nsp_registry: NamespaceRegistry,
    transport_protocol: TransportProtocol,
    receiver: mpsc::Receiver<JaResponse>,
    sessions: HashMap<u64, WeakJaSession>,
    transaction_manager: TransactionManager,
}

pub struct InnerConnection {
    shared: Shared,
    safe: Mutex<SafeShared>,
}

#[derive(Clone)]
pub struct JaConnection(Arc<InnerConnection>);

impl Deref for JaConnection {
    type Target = Arc<InnerConnection>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JaConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl JaConnection {
    /// Async task to handle demultiplexing of the inbound stream
    async fn demux_task(
        inbound_stream: mpsc::Receiver<String>,
        nsp_registry: NamespaceRegistry,
        transaction_manager: TransactionManager,
        root_namespace: &str,
    ) -> JaResult<()> {
        let mut stream = inbound_stream;
        while let Some(next) = stream.recv().await {
            let message = serde_json::from_str::<JaResponse>(&next)?;

            // Check if we have a pending transaction and demux to the proper namespace
            if let Some(pending) = message
                .transaction
                .clone()
                .and_then(|x| transaction_manager.get(&x))
            {
                nsp_registry.publish(&pending.namespace, message).await?;
                transaction_manager.success_close(&pending.id);
                continue;
            }

            // Try get the namespace from the response
            if let Some(namespace) = get_subnamespace_from_response(message.clone()) {
                let namespace = format!("{root_namespace}/{namespace}");
                nsp_registry.publish(&namespace, message).await?;
                continue;
            }

            // Fallback to publishing on the root namespace
            nsp_registry.publish(root_namespace, message).await?;
        }
        Ok(())
    }

    pub(crate) async fn open(config: JaConfig, transport: impl Transport) -> JaResult<Self> {
        let mut nsp_registry = NamespaceRegistry::new();
        let transaction_manager = TransactionManager::new();

        let root_namespace = config.root_namespace.clone();
        let namespace_receiver = nsp_registry.create_namespace(&root_namespace.clone());
        let (transport_protocol, receiver) =
            TransportProtocol::connect(transport, &config.uri).await?;

        let demux_join_handle = tokio::spawn({
            let nsp_registry = nsp_registry.clone();
            let transaction_manager = transaction_manager.clone();
            async move {
                JaConnection::demux_task(
                    receiver,
                    nsp_registry,
                    transaction_manager,
                    &root_namespace.clone(),
                )
                .await
            }
        });

        let shared = Shared {
            demux_abort_handle: demux_join_handle.abort_handle(),
            config,
        };
        let safe = SafeShared {
            nsp_registry,
            transport_protocol,
            receiver: namespace_receiver,
            sessions: HashMap::new(),
            transaction_manager,
        };
        let connection = Arc::new(InnerConnection {
            shared,
            safe: Mutex::new(safe),
        });
        Ok(Self(connection))
    }

    /// Creates a new session with janus server.
    pub async fn create(&mut self, ka_interval: u32) -> JaResult<JaSession> {
        log::info!("Creating new session");

        let request = json!({
            "janus": JaConnectionRequestProtocol::CreateSession,
        });

        self.send_request(request).await?;
        let response = { self.safe.lock().await.receiver.recv().await.unwrap() };
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

        let channel = self.create_subnamespace(&format!("{session_id}")).await;

        let session = JaSession::new(self.clone(), channel, session_id, ka_interval).await;
        self.safe
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
        let response = { self.safe.lock().await.receiver.recv().await.unwrap() };
        Ok(response)
    }

    pub(crate) async fn send_request(&mut self, request: Value) -> JaResult<()> {
        let request = self.decorate_request(request);
        let message = serde_json::to_string(&request)?;

        let (Some(janus_request), Some(transaction)) =
            (request["janus"].as_str(), request["transaction"].as_str())
        else {
            log::error!("Bad request body");
            return Err(JaError::InvalidJanusRequest);
        };

        let root_namespace = self.shared.config.root_namespace.clone();
        let namespace = match get_subnamespace_from_request(&request) {
            Some(namespace) => format!("{root_namespace}/{namespace}"),
            None => root_namespace,
        };

        let mut guard = self.safe.lock().await;
        guard
            .transaction_manager
            .create_transaction(transaction, janus_request, &namespace);
        guard.transport_protocol.send(message.as_bytes()).await
    }

    fn decorate_request(&self, mut request: Value) -> Value {
        let transaction = generate_transaction();
        request["apisecret"] = self.shared.config.apisecret.clone().into();
        request["transaction"] = transaction.into();
        request
    }

    pub(crate) async fn create_subnamespace(&self, namespace: &str) -> mpsc::Receiver<JaResponse> {
        self.safe
            .lock()
            .await
            .nsp_registry
            .create_namespace(&format!(
                "{}/{}",
                self.shared.config.root_namespace, namespace
            ))
    }
}

impl Drop for InnerConnection {
    fn drop(&mut self) {
        log::trace!("Connection dropped");
        self.shared.demux_abort_handle.abort();
    }
}
