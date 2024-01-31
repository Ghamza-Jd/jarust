use crate::jaconfig::CHANNEL_BUFFER_SIZE;
use crate::japrotocol::JaHandleRequestProtocol;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaResponseProtocol;
use crate::japrotocol::Jsep;
use crate::jasession::JaSession;
use crate::prelude::*;
use serde_json::json;
use serde_json::Value;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Weak;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::AbortHandle;

struct Shared {
    id: u64,
    session: JaSession,
    abort_handle: AbortHandle,
}

struct Exclusive {
    ack_receiver: mpsc::Receiver<JaResponse>,
}

pub struct InnerHandle {
    shared: Shared,
    exclusive: Mutex<Exclusive>,
}

#[derive(Clone)]
pub struct JaHandle(Arc<InnerHandle>);

#[derive(Debug)]
pub struct WeakJaHandle(Weak<InnerHandle>);

impl WeakJaHandle {
    pub(crate) fn _upgarde(&self) -> Option<JaHandle> {
        self.0.upgrade().map(JaHandle)
    }
}

impl Deref for JaHandle {
    type Target = Arc<InnerHandle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JaHandle {
    pub(crate) fn new(
        session: JaSession,
        mut receiver: mpsc::Receiver<JaResponse>,
        id: u64,
    ) -> (Self, mpsc::Receiver<JaResponse>) {
        let (ack_sender, ack_receiver) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        let (event_sender, event_receiver) = mpsc::channel(CHANNEL_BUFFER_SIZE);

        let join_handle = tokio::spawn(async move {
            while let Some(item) = receiver.recv().await {
                match item.janus {
                    JaResponseProtocol::Ack => {
                        ack_sender
                            .send(item.clone())
                            .await
                            .expect("Ack channel closed");
                    }
                    JaResponseProtocol::Event { .. } => {
                        event_sender.send(item).await.expect("Event channel closed");
                    }
                    _ => {}
                }
            }
        });

        let shared = Shared {
            id,
            session,
            abort_handle: join_handle.abort_handle(),
        };
        let safe = Exclusive { ack_receiver };

        (
            Self(Arc::new(InnerHandle {
                shared,
                exclusive: Mutex::new(safe),
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

    pub async fn message_with_jsep(&self, body: Value, jsep: Jsep) -> JaResult<JaResponse> {
        let request = json!({
            "janus": JaHandleRequestProtocol::Message,
            "body": body,
            "jsep": jsep
        });
        self.send_request(request).await?;
        let response = match self.exclusive.lock().await.ack_receiver.recv().await {
            Some(response) => response,
            None => {
                log::error!("Incomplete packet");
                return Err(JaError::IncompletePacket);
            }
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

impl Drop for InnerHandle {
    fn drop(&mut self) {
        log::trace!("Dropping handle {{ id: {} }}", self.shared.id);
        self.shared.abort_handle.abort();
    }
}
