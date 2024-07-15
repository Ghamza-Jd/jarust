use crate::jahandle::JaHandle;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::napmap::NapMap;
use crate::nw::jatransport::JaTransport;
use crate::params::AttachHandleParams;
use crate::prelude::*;
use async_trait::async_trait;
use jarust_rt::JaTask;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;

#[derive(Debug)]
pub struct Shared {
    id: u64,
    rsp_map: Arc<NapMap<String, JaResponse>>,
    transport: JaTransport,
}

#[derive(Debug)]
pub struct Exclusive {
    tasks: Vec<JaTask>,
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

impl JaSession {
    pub(crate) async fn new(
        mut receiver: JaResponseStream,
        id: u64,
        ka_interval: u32,
        capacity: usize,
        transport: JaTransport,
    ) -> Self {
        let rsp_map = Arc::new(NapMap::new(capacity));

        let rsp_cache_task = jarust_rt::spawn({
            let rsp_map = rsp_map.clone();
            async move {
                while let Some(rsp) = receiver.recv().await {
                    if let Some(transaction) = rsp.transaction.clone() {
                        rsp_map.insert(transaction, rsp).await;
                    }
                }
            }
        });

        let shared = Shared {
            id,
            rsp_map,
            transport,
        };
        let safe = Exclusive {
            tasks: vec![rsp_cache_task],
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

        session
            .inner
            .exclusive
            .lock()
            .await
            .tasks
            .push(keepalive_task);

        session
    }

    pub(crate) async fn send_request(&self, mut request: Value) -> JaResult<String> {
        request["session_id"] = self.inner.shared.id.into();
        self.inner.shared.transport.send(request).await
    }

    #[tracing::instrument(skip(self))]
    async fn keep_alive(self, ka_interval: u32) -> JaResult<()> {
        let mut interval = time::interval(Duration::from_secs(ka_interval.into()));
        let id = { self.inner.shared.id };
        loop {
            interval.tick().await;
            tracing::debug!("Sending {{ id: {id} }}");
            let transaction = self
                .send_request(json!({
                    "janus": "keepalive"
                }))
                .await?;
            let _ = self
                .poll_response(&transaction, Duration::from_secs(ka_interval.into()))
                .await?;
            tracing::debug!("OK");
        }
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
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
        self.tasks.iter().for_each(|task| {
            task.cancel();
        });
    }
}

#[async_trait]
impl Attach for JaSession {
    /// Attach a plugin to the current session
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    async fn attach(&self, params: AttachHandleParams) -> JaResult<(JaHandle, JaResponseStream)> {
        tracing::info!("Attaching new handle");

        let request = json!({
            "janus": "attach",
            "plugin": params.plugin_id,
        });

        let transaction = self.send_request(request).await?;
        let response = self.poll_response(&transaction, params.timeout).await?;

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

        let receiver = self
            .inner
            .shared
            .transport
            .add_handle_subroute(self.inner.shared.id, handle_id)
            .await;

        let (handle, event_receiver) =
            JaHandle::new(self.clone(), receiver, handle_id, params.capacity);

        tracing::info!("Handle created {{ handle_id: {handle_id} }}");

        Ok((handle, event_receiver))
    }
}

impl JaSession {
    pub async fn destory(&self, timeout: Duration) -> JaResult<()> {
        tracing::info!("Destroying session");

        let request = json!({
            "janus": "destroy"
        });

        let transaction = self.send_request(request).await?;
        let _ = self.poll_response(&transaction, timeout).await?;

        Ok(())
    }
}
