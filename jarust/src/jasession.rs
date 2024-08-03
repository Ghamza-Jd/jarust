use crate::jahandle::JaHandle;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::nw::jatransport::JaTransport;
use crate::params::AttachHandleParams;
use crate::prelude::*;
use async_trait::async_trait;
use jarust_rt::JaTask;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;

#[derive(Debug)]
pub struct Shared {
    id: u64,
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
    pub(crate) async fn new(id: u64, ka_interval: u32, transport: JaTransport) -> Self {
        let shared = Shared { id, transport };
        let safe = Exclusive { tasks: vec![] };

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

    #[tracing::instrument(skip(self))]
    async fn keep_alive(self, ka_interval: u32) -> JaResult<()> {
        let mut interval = time::interval(Duration::from_secs(ka_interval.into()));
        let id = { self.inner.shared.id };
        loop {
            interval.tick().await;
            tracing::debug!("Sending {{ id: {id} }}");
            let _ = self
                .inner
                .shared
                .transport
                .keep_alive(id, Duration::from_secs(ka_interval.into()))
                .await;
            tracing::debug!("OK");
        }
    }
}

impl JaSession {
    pub async fn destory(&self, timeout: Duration) -> JaResult<()> {
        tracing::info!("Destroying session");
        let session_id = self.inner.shared.id;
        self.inner
            .shared
            .transport
            .destory(session_id, timeout)
            .await
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

        let session_id = self.inner.shared.id;
        let response = self
            .inner
            .shared
            .transport
            .attach(session_id, params.plugin_id, params.timeout)
            .await?;

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

        let (handle, event_receiver) = JaHandle::new(
            handle_id,
            self.inner.shared.id,
            self.inner.shared.transport.clone(),
        )
        .await;

        tracing::info!("Handle created {{ handle_id: {handle_id} }}");

        Ok((handle, event_receiver))
    }
}
