use crate::jaconnection::JaConnection;
use crate::jahandle::JaHandle;
use crate::jahandle::WeakJaHandle;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaResponseProtocol;
use crate::japrotocol::JaSessionRequestProtocol;
use crate::japrotocol::JaSuccessProtocol;
use crate::jatask;
use crate::jatask::AbortHandle;
use crate::prelude::*;
use async_trait::async_trait;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Weak;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::time;

#[derive(Debug)]
pub struct Shared {
    id: u64,
    connection: JaConnection,
}

#[derive(Debug)]
pub struct Exclusive {
    receiver: mpsc::Receiver<JaResponse>,
    handles: HashMap<u64, WeakJaHandle>,
    abort_handle: Option<AbortHandle>,
}

#[derive(Debug)]
struct InnerSession {
    shared: Shared,
    exclusive: Mutex<Exclusive>,
}

#[derive(Clone, Debug)]
pub struct JaSession {
    inner: Arc<InnerSession>,
}

#[derive(Debug)]
pub struct WeakJaSession {
    _inner: Weak<InnerSession>,
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

        let session = Self {
            inner: Arc::new(InnerSession {
                shared,
                exclusive: Mutex::new(safe),
            }),
        };

        let this = session.clone();

        let abort_handle = jatask::spawn(async move {
            let _ = this.keep_alive(ka_interval).await;
        });

        session.inner.exclusive.lock().await.abort_handle = Some(abort_handle);

        session
    }

    pub(crate) async fn send_request(&self, mut request: Value) -> JaResult<()> {
        let mut connection = self.inner.shared.connection.clone();
        request["session_id"] = self.inner.shared.id.into();
        connection.send_request(request).await
    }

    async fn keep_alive(self, ka_interval: u32) -> JaResult<()> {
        let mut interval = time::interval(Duration::from_secs(ka_interval.into()));
        let id = { self.inner.shared.id };
        loop {
            interval.tick().await;
            log::trace!("Sending keep-alive {{ id: {id}, timeout: {ka_interval}s }}");
            self.send_request(json!({
                "janus": JaSessionRequestProtocol::KeepAlive,
            }))
            .await?;
            let _ = match self.inner.exclusive.lock().await.receiver.recv().await {
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
        WeakJaSession {
            _inner: Arc::downgrade(&self.inner),
        }
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
    /// Attach a plugin to the current session
    async fn attach(&self, plugin_id: &str) -> JaResult<(JaHandle, mpsc::Receiver<JaResponse>)> {
        log::info!("Attaching new handle {{ id: {} }}", self.inner.shared.id);

        let request = json!({
            "janus": JaSessionRequestProtocol::AttachPlugin,
            "plugin": plugin_id,
        });

        self.send_request(request).await?;

        let response = match self.inner.exclusive.lock().await.receiver.recv().await {
            Some(response) => response,
            None => {
                log::error!("Incomplete packet");
                return Err(JaError::IncompletePacket);
            }
        };

        let handle_id = match response.janus {
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

        let connection = self.inner.shared.connection.clone();

        let receiver = connection
            .add_subroute(&format!("{}/{}", self.inner.shared.id, handle_id))
            .await;

        let (handle, event_receiver) = JaHandle::new(self.clone(), receiver, handle_id);

        self.inner
            .exclusive
            .lock()
            .await
            .handles
            .insert(handle_id, handle.downgrade());

        log::info!("Handle created {{ id: {handle_id} }}");

        Ok((handle, event_receiver))
    }
}
