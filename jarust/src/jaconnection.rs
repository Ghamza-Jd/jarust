use crate::demux::Demux;
use crate::jaconfig::JaConfig;
use crate::japrotocol::JaConnectionRequestProtocol;
use crate::jasession::JaSession;
use crate::prelude::JaResult;
use crate::transport::wss::WebsocketTransport;
use crate::utils::generate_transaction;
use futures_util::StreamExt;
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
    transport: WebsocketTransport,
    receiver: mpsc::Receiver<String>,
    sessions: HashMap<u64, JaSession>,
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
    pub(crate) async fn open(config: JaConfig) -> JaResult<Self> {
        let mut demux = Demux::new();
        let root_namespace = config.root_namespace.clone();
        let namespace_receiver = demux.create_namespace(&root_namespace.clone());
        let (transport, receiver) = WebsocketTransport::connect(&config.uri).await?;
        let demux_clone = demux.clone();
        tokio::runtime::Handle::current().spawn(async move {
            let mut stream = receiver;
            while let Some(Ok(next)) = stream.next().await {
                let response: Value = serde_json::from_str(&next.to_string()).unwrap();

                if let Some(session_id) = response["session_id"].as_u64() {
                    let namespace = format!("{}/{session_id}", root_namespace.clone());
                    demux_clone
                        .publish(&namespace, next.to_string())
                        .await
                        .unwrap();
                    continue;
                }

                demux_clone
                    .publish(&root_namespace, next.to_string())
                    .await
                    .unwrap();
            }
        });

        let shared = Shared { config };
        let safe = SafeShared {
            demux,
            transport,
            receiver: namespace_receiver,
            sessions: HashMap::new(),
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

        log::info!("Session created (id={})", session_id);

        Ok(session)
    }

    pub(crate) async fn send_request(&mut self, request: Value) -> JaResult<()> {
        let request = self.decorate_request(request);
        let message = serde_json::to_string(&request)?;
        self.safe.lock().await.transport.send(&message).await
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
