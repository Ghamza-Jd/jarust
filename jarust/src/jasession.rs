use crate::dto::response::AttachResponse;
use crate::jaconnection::JaConnection;
use crate::jahandle::JaHandle;
use crate::jahandle::WeakJaHandle;
use crate::japrotocol::JaSessionRequestProtocol;
use crate::prelude::*;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Weak;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time;

pub struct Shared {
    id: u64,
    connection: JaConnection,
}

pub struct SafeShared {
    receiver: mpsc::Receiver<String>,
    handles: HashMap<u64, WeakJaHandle>,
    join_handle: Option<JoinHandle<()>>,
}

pub struct InnerSession {
    shared: Shared,
    safe: Mutex<SafeShared>,
}

#[derive(Clone)]
pub struct JaSession(Arc<InnerSession>);

pub struct WeakJaSession(Weak<InnerSession>);

impl WeakJaSession {
    pub(crate) fn _upgarde(&self) -> Option<JaSession> {
        self.0.upgrade().map(JaSession)
    }
}

impl std::ops::Deref for JaSession {
    type Target = Arc<InnerSession>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JaSession {
    pub async fn new(
        connection: JaConnection,
        receiver: mpsc::Receiver<String>,
        id: u64,
        ka_interval: u32,
    ) -> Self {
        let shared = Shared { id, connection };
        let safe = SafeShared {
            receiver,
            handles: HashMap::new(),
            join_handle: None,
        };

        let session = Self(Arc::new(InnerSession {
            shared,
            safe: Mutex::new(safe),
        }));

        let this = session.clone();

        let join_handle = tokio::spawn(async move {
            let _ = this.keep_alive(ka_interval).await;
        });

        session.safe.lock().await.join_handle = Some(join_handle);

        session
    }

    pub async fn attach(&self, plugin_id: &str) -> JaResult<(JaHandle, mpsc::Receiver<String>)> {
        log::info!("Attaching new handle {{ id: {} }}", self.shared.id);

        let request = json!({
            "janus": JaSessionRequestProtocol::AttachPlugin,
            "plugin": plugin_id,
        });

        self.send_request(request).await?;
        let response = {
            let mut guard = self.safe.lock().await;
            guard.receiver.recv().await.unwrap()
        };

        let response = serde_json::from_str::<AttachResponse>(&response)?;
        let handle_id = response.data.id;

        let connection = self.shared.connection.clone();

        let receiver = connection
            .create_subnamespace(&format!("{}/{}", self.shared.id, handle_id))
            .await;

        let (handle, event_receiver) = JaHandle::new(self.clone(), receiver, handle_id);

        self.safe
            .lock()
            .await
            .handles
            .insert(handle_id, handle.downgrade());

        log::info!("Handle created {{ id: {handle_id} }}");

        Ok((handle, event_receiver))
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
            self.safe.lock().await.receiver.recv().await.unwrap();
            log::trace!("keep-alive OK {{ id: {id} }}");
        }
    }

    pub(crate) fn downgrade(&self) -> WeakJaSession {
        WeakJaSession(Arc::downgrade(self))
    }
}

impl Drop for SafeShared {
    fn drop(&mut self) {
        if let Some(join_handle) = self.join_handle.take() {
            join_handle.abort();
            log::trace!("Keepalive task aborted");
        }
    }
}
