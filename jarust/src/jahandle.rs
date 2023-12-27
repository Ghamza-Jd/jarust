use crate::japrotocol::JaHandleRequestProtocol;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaResponseProtocol;
use crate::japrotocol::Jsep;
use crate::jasession::JaSession;
use crate::prelude::*;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use std::sync::Weak;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

struct Shared {
    id: u64,
    session: JaSession,
}

struct SafeShared {
    ack_receiver: mpsc::Receiver<String>,
}

pub struct InnerHandle {
    shared: Shared,
    safe: Mutex<SafeShared>,
}

#[derive(Clone)]
pub struct JaHandle(Arc<InnerHandle>);

pub struct WeakJaHandle(Weak<InnerHandle>);

impl WeakJaHandle {
    pub(crate) fn _upgarde(&self) -> Option<JaHandle> {
        self.0.upgrade().map(JaHandle)
    }
}

impl std::ops::Deref for JaHandle {
    type Target = Arc<InnerHandle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JaHandle {
    pub(crate) fn new(
        session: JaSession,
        mut receiver: mpsc::Receiver<String>,
        id: u64,
    ) -> (Self, mpsc::Receiver<String>) {
        let shared = Shared { id, session };
        let (ack_sender, ack_receiver) = mpsc::channel(100);
        let (event_sender, event_receiver) = mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(item) = receiver.recv().await {
                let response_type = serde_json::from_str::<JaResponse>(&item).unwrap();
                match response_type.janus {
                    JaResponseProtocol::Status(_) | JaResponseProtocol::Ack(_) => {
                        ack_sender.send(item.clone()).await.unwrap();
                    }
                    JaResponseProtocol::Event(_) => {
                        event_sender.send(item).await.unwrap();
                    }
                }
            }
        });

        let safe = SafeShared { ack_receiver };

        (
            Self(Arc::new(InnerHandle {
                shared,
                safe: Mutex::new(safe),
            })),
            event_receiver,
        )
    }

    async fn send_request(&self, mut request: Value) -> JaResult<()> {
        let session = self.shared.session.clone();
        request["handle_id"] = self.shared.id.into();
        session.send_request(request).await
    }

    pub async fn message(&self, body: Value) -> JaResult<()> {
        let request = json!({
            "janus": JaHandleRequestProtocol::Message,
            "body": body
        });
        self.send_request(request).await
    }

    pub async fn message_with_jsep(&self, body: Value, jsep: Jsep) -> JaResult<String> {
        let request = json!({
            "janus": JaHandleRequestProtocol::Message,
            "body": body,
            "jsep": jsep
        });
        self.send_request(request).await?;
        let response = {
            let mut guard = self.safe.lock().await;
            guard.ack_receiver.recv().await.unwrap()
        };
        Ok(response)
    }

    pub async fn detach(&self) -> JaResult<()> {
        log::info!("Detaching handle {{ id: {} }}", self.shared.id);
        let request = json!({
            "janus": JaHandleRequestProtocol::DetachPlugin,
        });
        self.send_request(request).await?;
        // let session = self.shared.session.clone();
        Ok(())
    }

    pub(crate) fn downgrade(&self) -> WeakJaHandle {
        WeakJaHandle(Arc::downgrade(self))
    }
}
