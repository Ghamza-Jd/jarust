use crate::jaconfig::CHANNEL_BUFFER_SIZE;
use crate::japrotocol::EstablishmentProtocol;
use crate::japrotocol::JaHandleRequestProtocol;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaResponseProtocol;
use crate::japrotocol::JaSuccessProtocol;
use crate::jasession::JaSession;
use crate::jatask;
use crate::jatask::AbortHandle;
use crate::prelude::*;
use serde::de::DeserializeOwned;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use std::sync::Weak;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

struct Shared {
    id: u64,
    session: JaSession,
    abort_handle: AbortHandle,
}

struct Exclusive {
    ack_receiver: mpsc::Receiver<JaResponse>,
    result_receiver: mpsc::Receiver<JaResponse>,
}

struct InnerHandle {
    shared: Shared,
    exclusive: Mutex<Exclusive>,
}

#[derive(Clone)]
pub struct JaHandle {
    inner: Arc<InnerHandle>,
}

#[derive(Debug)]
pub struct WeakJaHandle {
    _inner: Weak<InnerHandle>,
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

        let abort_handle = jatask::spawn(async move {
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
            abort_handle,
        };
        let exclusive = Exclusive {
            ack_receiver,
            result_receiver,
        };

        (
            Self {
                inner: Arc::new(InnerHandle {
                    shared,
                    exclusive: Mutex::new(exclusive),
                }),
            },
            event_receiver,
        )
    }

    async fn send_request(&self, mut request: Value) -> JaResult<()> {
        let session = self.inner.shared.session.clone();
        request["handle_id"] = self.inner.shared.id.into();
        session.send_request(request).await
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(self), fields(id = self.inner.shared.id))]
    async fn poll_response(&self, timeout: Duration) -> JaResult<JaResponse> {
        tracing::trace!("Polling response");
        let response = match tokio::time::timeout(
            timeout,
            self.inner.exclusive.lock().await.result_receiver.recv(),
        )
        .await
        {
            Ok(Some(response)) => response,
            Ok(None) => {
                tracing::error!("Incomplete packet");
                return Err(JaError::IncompletePacket);
            }
            Err(_) => {
                tracing::error!("Request timeout");
                return Err(JaError::RequestTimeout);
            }
        };
        Ok(response)
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(self), fields(id = self.inner.shared.id))]
    async fn poll_ack(&self, timeout: Duration) -> JaResult<JaResponse> {
        tracing::trace!("Polling ack");
        let response = match tokio::time::timeout(
            timeout,
            self.inner.exclusive.lock().await.ack_receiver.recv(),
        )
        .await
        {
            Ok(Some(response)) => response,
            Ok(None) => {
                tracing::error!("Incomplete packet");
                return Err(JaError::IncompletePacket);
            }
            Err(_) => {
                tracing::error!("Request timeout");
                return Err(JaError::RequestTimeout);
            }
        };
        Ok(response)
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
    pub async fn message_with_result<R>(&self, body: Value, timeout: Duration) -> JaResult<R>
    where
        R: DeserializeOwned,
    {
        let request = json!({
            "janus": JaHandleRequestProtocol::Message,
            "body": body
        });
        self.send_request(request).await?;
        let response = self.poll_response(timeout).await?;

        let result = match response.janus {
            JaResponseProtocol::Success(JaSuccessProtocol::Plugin { plugin_data }) => {
                match serde_json::from_value::<R>(plugin_data) {
                    Ok(result) => result,
                    Err(error) => {
                        tracing::error!("Failed to parse with error {error:#?}");
                        return Err(JaError::UnexpectedResponse);
                    }
                }
            }
            _ => {
                tracing::error!("Request failed");
                return Err(JaError::UnexpectedResponse);
            }
        };

        Ok(result)
    }

    /// Send a message and wait for the ack
    pub async fn message_with_ack(&self, body: Value, timeout: Duration) -> JaResult<JaResponse> {
        let request = json!({
            "janus": JaHandleRequestProtocol::Message,
            "body": body
        });
        self.send_request(request).await?;
        let response = self.poll_ack(timeout).await?;
        Ok(response)
    }

    /// Send a message with a specific establishment protocol and wait for the ack
    pub async fn message_with_establishment_protocol(
        &self,
        body: Value,
        protocol: EstablishmentProtocol,
        timeout: Duration,
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
        let response = self.poll_ack(timeout).await?;

        Ok(response)
    }

    pub async fn detach(&self) -> JaResult<()> {
        tracing::info!("Detaching handle {{ id: {} }}", self.inner.shared.id);
        let request = json!({
            "janus": JaHandleRequestProtocol::DetachPlugin,
        });
        self.send_request(request).await?;
        // let session = self.shared.session.clone();
        Ok(())
    }

    pub(crate) fn downgrade(&self) -> WeakJaHandle {
        WeakJaHandle {
            _inner: Arc::downgrade(&self.inner),
        }
    }
}

impl Drop for InnerHandle {
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self), fields(id = self.shared.id))]
    fn drop(&mut self) {
        tracing::debug!("Handle Dropped");
        self.shared.abort_handle.abort();
    }
}
