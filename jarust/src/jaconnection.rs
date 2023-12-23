use crate::demux::Demux;
use crate::jaconfig::JaConfig;
use crate::japrotocol::JaConnectionRequestProtocol;
use crate::jasession::JaSession;
use crate::prelude::*;
use crate::tmanager::TransactionManager;
use crate::transport::trans::Transport;
use crate::transport::trans::TransportProtocol;
use crate::utils::generate_transaction;
use crate::utils::get_subnamespace_from_request;
use crate::utils::get_subnamespace_from_response;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Weak;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub struct Shared {
    config: JaConfig,
}

pub struct SafeShared {
    demux: Demux,
    transport_protocol: TransportProtocol,
    receiver: mpsc::Receiver<String>,
    sessions: HashMap<u64, JaSession>,
    transaction_manager: TransactionManager,
}

pub struct InnerConnection {
    shared: Shared,
    safe: Mutex<SafeShared>,
}

#[derive(Clone)]
pub struct JaConnection(Arc<InnerConnection>);

impl std::ops::Deref for JaConnection {
    type Target = Arc<InnerConnection>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for JaConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct WeakJaConnection(Weak<InnerConnection>);

impl WeakJaConnection {
    pub(crate) fn upgarde(&self) -> Option<JaConnection> {
        self.0.upgrade().map(JaConnection)
    }
}

impl JaConnection {
    /// Async task to handle demultiplexing of the inbound stream
    async fn demux_task(
        inbound_stream: mpsc::Receiver<String>,
        demux: Demux,
        transaction_manager: TransactionManager,
        root_namespace: &str,
    ) -> JaResult<()> {
        let mut stream = inbound_stream;
        while let Some(next) = stream.recv().await {
            let response: Value = serde_json::from_str(&next.to_string()).unwrap();

            // Check if we have a pending transaction and demux to the proper namespace
            if let Some(pending) = response
                .get("transaction")
                .and_then(Value::as_str)
                .and_then(|x| transaction_manager.get(x))
            {
                demux.publish(&pending.namespace, next.to_string()).await?;
                transaction_manager.success_close(&pending.id);
                continue;
            }

            // Try get the namespace from the response
            if let Some(namespace) = get_subnamespace_from_response(&response) {
                let namespace = format!("{}/{}", root_namespace, namespace);
                demux.publish(&namespace, next.to_string()).await?;
                continue;
            }

            // Fallback to publishing on the root namespace
            demux.publish(root_namespace, next.to_string()).await?;
        }
        Ok(())
    }

    pub(crate) async fn open(config: JaConfig, transport: impl Transport) -> JaResult<Self> {
        let mut demux = Demux::new();
        let transaction_manager = TransactionManager::new();

        let root_namespace = config.root_namespace.clone();
        let namespace_receiver = demux.create_namespace(&root_namespace.clone());
        let (transport_protocol, receiver) =
            TransportProtocol::connect(transport, &config.uri).await?;

        let demux_clone = demux.clone();
        let transaction_manager_clone = transaction_manager.clone();
        tokio::runtime::Handle::current().spawn(async move {
            JaConnection::demux_task(
                receiver,
                demux_clone,
                transaction_manager_clone,
                &root_namespace.clone(),
            )
            .await
        });

        let shared = Shared { config };
        let safe = SafeShared {
            demux,
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

    pub async fn create(&mut self, ka_interval: u32) -> JaResult<JaSession> {
        log::info!("Creating new session");

        let request = json!({
            "janus": JaConnectionRequestProtocol::CreateSession,
        });

        self.send_request(request).await?;
        let res = { self.safe.lock().await.receiver.recv().await.unwrap() };
        let res = serde_json::from_str::<CreateSessionResponse>(&res)?;
        let session_id = res.data.id;

        let channel = self.create_subnamespace(&format!("{session_id}")).await;

        let session = JaSession::new(self.downgrade(), channel, session_id, ka_interval);
        self.safe
            .lock()
            .await
            .sessions
            .insert(session_id, session.clone());

        log::info!("Session created {{ id: {} }}", session_id);

        Ok(session)
    }

    pub async fn server_info(&mut self) -> JaResult<String> {
        let request = json!({
            "janus": JaConnectionRequestProtocol::ServerInfo,
        });

        self.send_request(request).await?;
        let res = { self.safe.lock().await.receiver.recv().await.unwrap() };
        Ok(res)
    }

    pub(crate) async fn send_request(&mut self, request: Value) -> JaResult<()> {
        let request = self.decorate_request(request);
        let message = serde_json::to_string(&request)?;

        let (Some(janus_request), Some(transaction)) =
            (request["janus"].as_str(), request["transaction"].as_str())
        else {
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

    pub(crate) fn decorate_request(&self, mut request: Value) -> Value {
        let transaction = generate_transaction();
        request["apisecret"] = self.shared.config.apisecret.clone().into();
        request["transaction"] = transaction.into();
        request
    }

    pub(crate) fn downgrade(&self) -> WeakJaConnection {
        WeakJaConnection(Arc::downgrade(self))
    }

    pub(crate) async fn create_subnamespace(&self, namespace: &str) -> mpsc::Receiver<String> {
        self.safe.lock().await.demux.create_namespace(&format!(
            "{}/{}",
            self.shared.config.root_namespace, namespace
        ))
    }
}

#[derive(Deserialize)]
struct CreateSessionResponse {
    data: CreateSessionInnerResponse,
}

#[derive(Deserialize)]
struct CreateSessionInnerResponse {
    id: u64,
}
