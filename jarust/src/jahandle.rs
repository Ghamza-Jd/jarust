use crate::japrotocol::EstablishmentProtocol;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::napmap::NapMap;
use crate::nw::jatransport::JaTransport;
use crate::prelude::*;
use jarust_rt::JaTask;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

struct Shared {
    id: u64,
    session_id: u64,
    task: JaTask,
    transport: JaTransport,
}

struct InnerHandle {
    shared: Shared,
}

#[derive(Clone)]
pub struct JaHandle {
    inner: Arc<InnerHandle>,
}

impl JaHandle {
    async fn demux_recv_stream(
        inbound_stream: JaResponseStream,
        ack_map: Arc<NapMap<String, JaResponse>>,
        rsp_map: Arc<NapMap<String, JaResponse>>,
        event_sender: mpsc::UnboundedSender<JaResponse>,
    ) {
        let mut stream = inbound_stream;
        while let Some(item) = stream.recv().await {
            match item.janus {
                ResponseType::Ack => {
                    if let Some(transaction) = item.transaction.clone() {
                        ack_map.insert(transaction, item).await;
                    }
                }
                ResponseType::Event { .. } => {
                    _ = event_sender.send(item);
                }
                ResponseType::Success(JaSuccessProtocol::Plugin { .. }) => {
                    if let Some(transaction) = item.transaction.clone() {
                        rsp_map.insert(transaction, item).await;
                    }
                }
                ResponseType::Error { .. } => {
                    if let Some(transaction) = item.transaction.clone() {
                        rsp_map.insert(transaction.clone(), item.clone()).await;
                        ack_map.insert(transaction, item).await;
                    }
                }
                _ => {}
            }
        }
    }

    pub(crate) async fn new(
        id: u64,
        session_id: u64,
        capacity: usize,
        transport: JaTransport,
    ) -> (Self, JaResponseStream) {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        let ack_map = Arc::new(NapMap::<String, JaResponse>::new(capacity));
        let rsp_map = Arc::new(NapMap::<String, JaResponse>::new(capacity));

        let receiver = transport.add_handle_subroute(session_id, id).await;

        let task = jarust_rt::spawn(JaHandle::demux_recv_stream(
            receiver,
            ack_map.clone(),
            rsp_map.clone(),
            event_sender,
        ));

        let shared = Shared {
            id,
            session_id,
            task,
            transport,
        };

        let jahandle = Self {
            inner: Arc::new(InnerHandle { shared }),
        };

        (jahandle, event_receiver)
    }

    async fn send_request(&self, mut request: Value) -> JaResult<String> {
        request["handle_id"] = self.inner.shared.id.into();
        request["session_id"] = self.inner.shared.session_id.into();
        self.inner.shared.transport.send(request).await
    }

    /// Send a one-shot message
    pub async fn fire_and_forget(&self, body: Value) -> JaResult<()> {
        let request = json!({
            "janus": "message",
            "body": body
        });
        self.send_request(request).await?;
        Ok(())
    }

    /// Send a message and wait for the expected response
    pub async fn send_waiton_rsp<R>(&self, body: Value, timeout: Duration) -> JaResult<R>
    where
        R: DeserializeOwned,
    {
        let request = json!({
            "janus": "message",
            "body": body
        });
        let transaction = self.send_request(request).await?;
        let response = self
            .inner
            .shared
            .transport
            .poll_response(&transaction, timeout)
            .await?;

        let result = match response.janus {
            ResponseType::Success(JaSuccessProtocol::Plugin { plugin_data }) => {
                match serde_json::from_value::<R>(plugin_data.data) {
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
    pub async fn send_waiton_ack(&self, body: Value, timeout: Duration) -> JaResult<JaResponse> {
        let request = json!({
            "janus": "message",
            "body": body
        });
        let transaction = self.send_request(request).await?;
        let response = self
            .inner
            .shared
            .transport
            .poll_ack(&transaction, timeout)
            .await?;
        Ok(response)
    }

    /// Send a message with a specific establishment protocol and wait for the ack
    pub async fn send_waiton_ack_with_establishment(
        &self,
        body: Value,
        protocol: EstablishmentProtocol,
        timeout: Duration,
    ) -> JaResult<JaResponse> {
        let request = match protocol {
            EstablishmentProtocol::JSEP(jsep) => json!({
                "janus": "message",
                "body": body,
                "jsep": jsep
            }),
            EstablishmentProtocol::RTP(rtp) => json!({
                "janus": "message",
                "body": body,
                "rtp": rtp
            }),
        };
        let transaction = self.send_request(request).await?;
        let response = self
            .inner
            .shared
            .transport
            .poll_ack(&transaction, timeout)
            .await?;
        Ok(response)
    }

    /// Send a one-shot message with a specific establishment protocol
    pub async fn fire_and_forget_with_establishment(
        &self,
        body: Value,
        protocol: EstablishmentProtocol,
    ) -> JaResult<()> {
        let request = match protocol {
            EstablishmentProtocol::JSEP(jsep) => json!({
                "janus": "message",
                "body": body,
                "jsep": jsep
            }),
            EstablishmentProtocol::RTP(rtp) => json!({
                "janus": "message",
                "body": body,
                "rtp": rtp
            }),
        };
        self.send_request(request).await?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct Candidate {
    #[serde(rename = "sdpMid")]
    pub sdp_mid: String,
    #[serde(rename = "sdpMLineIndex")]
    pub sdp_mline_index: String,
    pub candidate: String,
}

impl JaHandle {
    pub async fn hangup(&self, timeout: Duration) -> JaResult<()> {
        let request = json!({
            "janus": "hangup"
        });
        self.send_waiton_ack(request, timeout).await?;
        Ok(())
    }

    pub async fn detach(self, timeout: Duration) -> JaResult<()> {
        let request = json!({
            "janus": "detach"
        });
        self.send_waiton_ack(request, timeout).await?;
        Ok(())
    }

    pub async fn trickle_single_candidate(
        &self,
        candidate: Candidate,
        timeout: Duration,
    ) -> JaResult<()> {
        let request = json!({
            "janus": "trickle",
            "candidate": candidate
        });
        self.send_waiton_ack(request, timeout).await?;
        Ok(())
    }

    pub async fn trickle_candidates(
        &self,
        candidates: Vec<Candidate>,
        timeout: Duration,
    ) -> JaResult<()> {
        let request = json!({
            "janus": "trickle",
            "candidates": candidates
        });
        self.send_waiton_ack(request, timeout).await?;
        Ok(())
    }

    pub async fn complete_trickle(&self, timeout: Duration) -> JaResult<()> {
        let request = json!({
            "janus": "trickle",
            "candidate": {
                "completed" : true
            }
        });
        self.send_waiton_ack(request, timeout).await?;
        Ok(())
    }
}

impl Drop for InnerHandle {
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self), fields(id = self.shared.id))]
    fn drop(&mut self) {
        tracing::debug!("Handle Dropped");
        self.shared.task.cancel();
    }
}
