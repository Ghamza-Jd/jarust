use crate::jahandle::JaHandle;
use crate::jahandle::NewHandleParams;
use crate::prelude::*;
use async_trait::async_trait;
use jarust_interface::janus_interface::JanusInterfaceImpl;
use jarust_rt::JaTask;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::time;

#[derive(Debug)]
pub struct Shared {
    id: u64,
    interface: JanusInterfaceImpl,
}

#[derive(Debug, Default)]
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

pub struct NewSessionParams {
    pub session_id: u64,
    pub ka_interval: u32,
    pub interface: JanusInterfaceImpl,
}

impl JaSession {
    pub(crate) async fn new(params: NewSessionParams) -> Self {
        let shared = Shared {
            id: params.session_id,
            interface: params.interface,
        };
        let exclusive = Mutex::new(Exclusive::default());
        let session = Self {
            inner: Arc::new(InnerSession { shared, exclusive }),
        };

        let this = session.clone();

        let keepalive_task = jarust_rt::spawn("keepalive", async move {
            let _ = this.keep_alive(params.ka_interval).await;
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

    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.shared.id))]
    async fn keep_alive(self, ka_interval: u32) -> Result<(), jarust_interface::Error> {
        if !self.inner.shared.interface.has_keep_alive() {
            tracing::debug!("Keep-alive not supported");
            return Ok(());
        }

        let duration = Duration::from_secs(ka_interval.into());
        let mut interval = time::interval(duration);
        let id = { self.inner.shared.id };
        loop {
            interval.tick().await;
            tracing::debug!("Sending keep-alive");
            match self.inner.shared.interface.keep_alive(id, duration).await {
                Ok(_) => tracing::debug!("Keep-alive success"),
                Err(e) => tracing::error!("Keep-alive failed: {:?}", e),
            };
        }
    }
}

impl JaSession {
    /// Destroy the current session
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.shared.id))]
    pub async fn destroy(&self, timeout: Duration) -> Result<(), jarust_interface::Error> {
        tracing::info!("Destroying session");
        let session_id = self.inner.shared.id;
        self.inner
            .shared
            .interface
            .destroy(session_id, timeout)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Attach for JaSession {
    /// Attach a plugin to the current session
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.shared.id))]
    async fn attach(
        &self,
        plugin_id: String,
        timeout: Duration,
    ) -> Result<(JaHandle, mpsc::UnboundedReceiver<JaResponse>), jarust_interface::Error> {
        tracing::info!(plugin = &plugin_id, "Attaching new handle");
        let session_id = self.inner.shared.id;
        let (handle_id, event_receiver) = self
            .inner
            .shared
            .interface
            .attach(session_id, plugin_id, timeout)
            .await?;

        let handle = JaHandle::new(NewHandleParams {
            handle_id,
            session_id,
            interface: self.inner.shared.interface.clone(),
        })
        .await;
        tracing::info!(id = handle_id, "Handle created");
        Ok((handle, event_receiver))
    }
}

impl Drop for Exclusive {
    fn drop(&mut self) {
        self.tasks.iter().for_each(|task| {
            task.cancel();
        });
    }
}