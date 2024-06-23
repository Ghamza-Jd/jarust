use crate::jaconnection::JaConnection;
use crate::jahandle::JaHandle;
use crate::jahandle::WeakJaHandle;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::prelude::*;
use async_trait::async_trait;
use jarust_rt::JaTask;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Weak;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;

#[derive(Debug)]
pub struct Shared {
    id: u64,
    connection: JaConnection,
}

#[derive(Debug)]
pub struct Exclusive {
    receiver: JaResponseStream,
    handles: HashMap<u64, WeakJaHandle>,
    task: Option<JaTask>,
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
        receiver: JaResponseStream,
        id: u64,
        ka_interval: u32,
    ) -> Self {
        let shared = Shared { id, connection };
        let safe = Exclusive {
            receiver,
            handles: HashMap::new(),
            task: None,
        };

        let session = Self {
            inner: Arc::new(InnerSession {
                shared,
                exclusive: Mutex::new(safe),
            }),
        };

        let this = session.clone();

        let keepalive_task = jarust_rt::spawn(async move {
            let _ = this.keep_alive(ka_interval).await;
        });

        session.inner.exclusive.lock().await.task = Some(keepalive_task);

        session
    }

    pub(crate) async fn send_request(&self, mut request: Value) -> JaResult<String> {
        let mut connection = self.inner.shared.connection.clone();
        request["session_id"] = self.inner.shared.id.into();
        connection.send_request(request).await
    }

    #[tracing::instrument(skip(self))]
    async fn keep_alive(self, ka_interval: u32) -> JaResult<()> {
        let mut interval = time::interval(Duration::from_secs(ka_interval.into()));
        let id = { self.inner.shared.id };
        loop {
            interval.tick().await;
            tracing::debug!("Sending {{ id: {id} }}");
            self.send_request(json!({
                "janus": "keepalive"
            }))
            .await?;
            let _ = match self.inner.exclusive.lock().await.receiver.recv().await {
                Some(response) => response,
                None => {
                    tracing::error!("Incomplete packet");
                    return Err(JaError::IncompletePacket);
                }
            };
            tracing::debug!("OK");
        }
    }

    pub(crate) fn downgrade(&self) -> WeakJaSession {
        WeakJaSession {
            _inner: Arc::downgrade(&self.inner),
        }
    }
}

impl Drop for InnerSession {
    #[tracing::instrument(parent = None, level = tracing::Level::TRACE, skip(self), fields(id = self.shared.id))]
    fn drop(&mut self) {
        tracing::debug!("Session dropped");
    }
}

impl Drop for Exclusive {
    #[tracing::instrument(parent = None, level = tracing::Level::TRACE, skip(self))]
    fn drop(&mut self) {
        if let Some(join_handle) = self.task.take() {
            tracing::debug!("Keepalive task aborted");
            join_handle.cancel();
        }
    }
}

#[async_trait]
impl Attach for JaSession {
    /// Attach a plugin to the current session
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    async fn attach(
        &self,
        plugin_id: &str,
        capacity: usize,
    ) -> JaResult<(JaHandle, JaResponseStream)> {
        tracing::info!("Attaching new handle");

        let request = json!({
            "janus": "attach",
            "plugin": plugin_id,
        });

        self.send_request(request).await?;

        let response = match self.inner.exclusive.lock().await.receiver.recv().await {
            Some(response) => response,
            None => {
                tracing::error!("Incomplete packet");
                return Err(JaError::IncompletePacket);
            }
        };

        let handle_id = match response.janus {
            ResponseType::Success(JaSuccessProtocol::Data { data }) => data.id,
            ResponseType::Error { error } => {
                let what = JaError::JanusError {
                    code: error.code,
                    reason: error.reason,
                };
                tracing::error!("{what}");
                return Err(what);
            }
            _ => {
                tracing::error!("Unexpected response");
                return Err(JaError::UnexpectedResponse);
            }
        };

        let connection = self.inner.shared.connection.clone();

        let receiver = connection
            .add_subroute(&format!("{}/{}", self.inner.shared.id, handle_id))
            .await;

        let (handle, event_receiver) = JaHandle::new(self.clone(), receiver, handle_id, capacity);

        self.inner
            .exclusive
            .lock()
            .await
            .handles
            .insert(handle_id, handle.downgrade());

        tracing::info!("Handle created {{ handle_id: {handle_id} }}");

        Ok((handle, event_receiver))
    }
}
