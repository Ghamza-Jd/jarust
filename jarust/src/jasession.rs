use crate::jaconnection::WeakJaConnection;
use crate::jahandle::JaHandle;
use crate::japrotocol::JaSessionRequestProtocol;
use crate::prelude::*;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Weak;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::time;

pub struct Shared {
    id: u64,
    connection: WeakJaConnection,
}

pub struct SafeShared {
    receiver: mpsc::Receiver<String>,
    handles: HashMap<u64, JaHandle>,
}

pub struct InnerSession {
    shared: Shared,
    safe: Mutex<SafeShared>,
}

#[derive(Clone)]
pub struct JaSession(Arc<InnerSession>);

pub struct WeakJaSession(Weak<InnerSession>);

impl WeakJaSession {
    pub(crate) fn upgarde(&self) -> Option<JaSession> {
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
    pub fn new(
        connection: WeakJaConnection,
        receiver: mpsc::Receiver<String>,
        id: u64,
        ka_interval: u32,
    ) -> Self {
        let shared = Shared { id, connection };
        let safe = SafeShared {
            receiver,
            handles: HashMap::new(),
        };

        let session = Self(Arc::new(InnerSession {
            shared,
            safe: Mutex::new(safe),
        }));

        let this = session.clone();

        let handle = tokio::runtime::Handle::current();
        let _join_handle = handle.spawn(async move {
            let _ = this.keep_alive(ka_interval).await;
        });

        session
    }

    pub async fn attach(&self, plugin_id: &str) -> JaResult<JaHandle> {
        log::info!("[{}] attaching new handle", self.shared.id);

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

        let Some(connection) = self.shared.connection.upgarde() else {
            return Err(JaError::DanglingSession);
        };

        let receiver = connection
            .create_subnamespace(&format!("{}/{}", self.shared.id, handle_id))
            .await;

        let handle = JaHandle::new(self.downgrade(), receiver, handle_id);

        self.safe
            .lock()
            .await
            .handles
            .insert(handle_id, handle.clone());

        log::info!("Handle created (id={})", handle_id);

        Ok(handle)
    }

    pub(crate) async fn send_request(&self, mut request: Value) -> JaResult<()> {
        let Some(mut connection) = self.shared.connection.upgarde() else {
            log::trace!("[{}] dangling session, cleaning it up", self.shared.id);
            return Err(JaError::DanglingSession);
        };
        request["session_id"] = self.shared.id.into();
        connection.send_request(request).await
    }

    async fn keep_alive(self, ka_interval: u32) -> JaResult<()> {
        let mut interval = time::interval(Duration::from_secs(ka_interval.into()));
        let id = { self.shared.id };
        loop {
            interval.tick().await;
            log::trace!("[{}] sending keep-alive (timeout={}s)", id, ka_interval);
            self.send_request(json!({
                "janus": JaSessionRequestProtocol::KeepAlive,
            }))
            .await?;
            self.safe.lock().await.receiver.recv().await.unwrap();
            log::trace!("[{}] keep-alive OK", id);
        }
    }

    pub(crate) fn downgrade(&self) -> WeakJaSession {
        WeakJaSession(Arc::downgrade(self))
    }
}

#[derive(Deserialize)]
struct AttachResponse {
    data: AttachInnerResponse,
}

#[derive(Deserialize)]
struct AttachInnerResponse {
    id: u64,
}
