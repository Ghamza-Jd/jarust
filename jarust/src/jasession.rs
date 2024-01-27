use crate::jaconnection::JaConnection;
use crate::jahandle::JaHandle;
use crate::jahandle::WeakJaHandle;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaResponseProtocol;
use crate::japrotocol::JaSessionRequestProtocol;
use crate::prelude::*;
use async_trait::async_trait;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Weak;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::AbortHandle;
use tokio::time;

pub struct Shared {
    id: u64,
    connection: JaConnection,
}

pub struct Exclusive {
    receiver: mpsc::Receiver<JaResponse>,
    handles: HashMap<u64, WeakJaHandle>,
    abort_handle: Option<AbortHandle>,
}

pub struct InnerSession {
    shared: Shared,
    exclusive: Mutex<Exclusive>,
}

#[derive(Clone)]
pub struct JaSession(Arc<InnerSession>);

pub struct WeakJaSession(Weak<InnerSession>);

impl WeakJaSession {
    pub(crate) fn _upgarde(&self) -> Option<JaSession> {
        self.0.upgrade().map(JaSession)
    }
}

impl Deref for JaSession {
    type Target = Arc<InnerSession>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JaSession {
    pub async fn new(
        connection: JaConnection,
        receiver: mpsc::Receiver<JaResponse>,
        id: u64,
        ka_interval: u32,
    ) -> Self {
        let shared = Shared { id, connection };
        let safe = Exclusive {
            receiver,
            handles: HashMap::new(),
            abort_handle: None,
        };

        let session = Self(Arc::new(InnerSession {
            shared,
            exclusive: Mutex::new(safe),
        }));

        let this = session.clone();

        let join_handle = tokio::spawn(async move {
            let _ = this.keep_alive(ka_interval).await;
        });

        session.exclusive.lock().await.abort_handle = Some(join_handle.abort_handle());

        session
    }

    pub(crate) async fn send_request(&self, mut request: Value) -> JaResult<()> {
        let mut connection = self.shared.connection.clone();
        request["session_id"] = self.shared.id.into();
        connection.send_request(request).await
    }

    async fn keep_alive(self, ka_interval: u32) -> JaResult<()> {
        let mut interval = time::interval(Duration::from_secs(ka_interval.into()));
        let id = { self.shared.id };
        loop {
            interval.tick().await;
            log::trace!("Sending keep-alive {{ id: {id}, timeout: {ka_interval}s }}");
            self.send_request(json!({
                "janus": JaSessionRequestProtocol::KeepAlive,
            }))
            .await?;
            let _ = match self.exclusive.lock().await.receiver.recv().await {
                Some(response) => response,
                None => {
                    log::error!("Incomplete packet");
                    return Err(JaError::IncompletePacket);
                }
            };
            log::trace!("keep-alive OK {{ id: {id} }}");
        }
    }

    pub(crate) fn downgrade(&self) -> WeakJaSession {
        WeakJaSession(Arc::downgrade(self))
    }
}

impl Drop for Exclusive {
    fn drop(&mut self) {
        if let Some(join_handle) = self.abort_handle.take() {
            log::trace!("Keepalive task aborted");
            join_handle.abort();
        }
    }
}

#[async_trait]
impl Attach for JaSession {
    async fn attach(&self, plugin_id: &str) -> JaResult<(JaHandle, mpsc::Receiver<JaResponse>)> {
        log::info!("Attaching new handle {{ id: {} }}", self.shared.id);

        let request = json!({
            "janus": JaSessionRequestProtocol::AttachPlugin,
            "plugin": plugin_id,
        });

        self.send_request(request).await?;

        let response = match self.exclusive.lock().await.receiver.recv().await {
            Some(response) => response,
            None => {
                log::error!("Incomplete packet");
                return Err(JaError::IncompletePacket);
            }
        };

        let handle_id = match response.janus {
            JaResponseProtocol::Success { data } => data.id,
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

        let connection = self.shared.connection.clone();

        let receiver = connection
            .create_subnamespace(&format!("{}/{}", self.shared.id, handle_id))
            .await;

        let (handle, event_receiver) = JaHandle::new(self.clone(), receiver, handle_id);

        self.exclusive
            .lock()
            .await
            .handles
            .insert(handle_id, handle.downgrade());

        log::info!("Handle created {{ id: {handle_id} }}");

        Ok((handle, event_receiver))
    }
}
