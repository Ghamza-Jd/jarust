use crate::jaconfig::CHANNEL_BUFFER_SIZE;
use crate::japrotocol::EstablishmentProtocol;
use crate::japrotocol::JaHandleRequestProtocol;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaResponseProtocol;
use crate::japrotocol::JaSuccessProtocol;
use crate::jasession::JaSession;
use crate::prelude::*;
use serde::de::DeserializeOwned;
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
    result_receiver: mpsc::Receiver<JaResponse>,
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
        let (result_sender, result_receiver) = mpsc::channel(CHANNEL_BUFFER_SIZE);
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
                    JaResponseProtocol::Success(JaSuccessProtocol::Plugin { .. }) => {
                        result_sender
                            .send(item)
                            .await
                            .expect("Result channel closed");
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
        let exclusive = Exclusive {
            ack_receiver,
            result_receiver,
        };

        (
            Self(Arc::new(InnerHandle {
                shared,
                exclusive: Mutex::new(exclusive),
            })),
            event_receiver,
        )
    }

    async fn send_request(&self, mut request: Value) -> JaResult<()> {
        let session = self.shared.session.clone();
        request["handle_id"] = self.shared.id.into();
        session.send_request(request).await
    }

    /// Send a one-shot message
    pub async fn message(&self, body: Value) -> JaResult<()> {
        let request = json!({
            "janus": JaHandleRequestProtocol::Message,
            "body": body
        });
        self.send_request(request).await
    }

    /// Send a message and wait for the expected response
    pub async fn message_with_result<Result>(&self, body: Value) -> JaResult<Result>
    where
        Result: DeserializeOwned,
    {
        let request = json!({
            "janus": JaHandleRequestProtocol::Message,
            "body": body
        });
        self.send_request(request).await?;
        let response = {
            let mut guard = self.exclusive.lock().await;
            guard.result_receiver.recv().await.unwrap()
        };

        let result = match response.janus {
            JaResponseProtocol::Success(JaSuccessProtocol::Plugin { plugin_data }) => {
                match serde_json::from_value::<Result>(plugin_data) {
                    Ok(result) => result,
                    Err(error) => {
                        log::error!("Failed to parse with error {error:#?}");
                        return Err(JaError::UnexpectedResponse);
                    }
                }
            }
            _ => {
                log::error!("Request failed");
                return Err(JaError::UnexpectedResponse);
            }
        };

        Ok(result)
    }

    /// Send a message and wait for the ack
    pub async fn message_with_ack(&self, body: Value) -> JaResult<JaResponse> {
        let request = json!({
            "janus": JaHandleRequestProtocol::Message,
            "body": body
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

    /// Send a message with a specific establishment protocol and wait for the ack
    pub async fn message_with_establishment_protocol(
        &self,
        body: Value,
        protocol: EstablishmentProtocol,
    ) -> JaResult<JaResponse> {
        let request = match protocol {
            EstablishmentProtocol::JSEP(jsep) => json!({
                "janus": JaHandleRequestProtocol::Message,
                "body": body,
                "jsep": jsep
            }),
            EstablishmentProtocol::RTP(rtp) => json!({
                "janus": JaHandleRequestProtocol::Message,
                "body": body,
                "rtp": rtp
            }),
        };
        self.send_request(request).await?;
        let response = {
            let mut guard = self.exclusive.lock().await;
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

impl Drop for InnerHandle {
    fn drop(&mut self) {
        log::trace!("Dropping handle {{ id: {} }}", self.shared.id);
        self.shared.abort_handle.abort();
    }
}
