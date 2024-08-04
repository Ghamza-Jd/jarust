use crate::prelude::*;
use jarust_transport_next::handle_msg::HandleMessage;
use jarust_transport_next::handle_msg::HandleMessageWithEstablishment;
use jarust_transport_next::handle_msg::HandleMessageWithEstablishmentAndTimeout;
use jarust_transport_next::handle_msg::HandleMessageWithTimeout;
use jarust_transport_next::japrotocol::Candidate;
use jarust_transport_next::japrotocol::EstablishmentProtocol;
use jarust_transport_next::japrotocol::JaResponse;
use jarust_transport_next::jatransport::JaTransport;
use serde::de::DeserializeOwned;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;

struct InnerHandle {
    id: u64,
    session_id: u64,
    transport: JaTransport,
}

#[derive(Clone)]
pub struct JaHandle {
    inner: Arc<InnerHandle>,
}

impl JaHandle {
    pub(crate) async fn new(
        id: u64,
        session_id: u64,
        transport: JaTransport,
    ) -> (Self, JaResponseStream) {
        let receiver = transport.add_handle_subroute(session_id, id).await;

        let jahandle = Self {
            inner: Arc::new(InnerHandle {
                id,
                session_id,
                transport,
            }),
        };

        (jahandle, receiver)
    }

    /// Send a one-shot message
    pub async fn fire_and_forget(&self, body: Value) -> JaResult<()> {
        self.inner
            .transport
            // .fire_and_forget_msg(self.inner.session_id, self.inner.id, body)
            .fire_and_forget_msg(HandleMessage {
                session_id: self.inner.session_id,
                handle_id: self.inner.id,
                body,
            })
            .await?;
        Ok(())
    }

    /// Send a message and wait for the expected response
    pub async fn send_waiton_rsp<R>(&self, body: Value, timeout: Duration) -> JaResult<R>
    where
        R: DeserializeOwned,
    {
        let res = self
            .inner
            .transport
            .send_msg_waiton_rsp(HandleMessageWithTimeout {
                session_id: self.inner.session_id,
                handle_id: self.inner.id,
                body,
                timeout,
            })
            .await?;
        Ok(res)
    }

    /// Send a message and wait for the ack
    pub async fn send_waiton_ack(&self, body: Value, timeout: Duration) -> JaResult<JaResponse> {
        let ack = self
            .inner
            .transport
            .send_msg_waiton_ack(HandleMessageWithTimeout {
                session_id: self.inner.session_id,
                handle_id: self.inner.id,
                body,
                timeout,
            })
            .await?;
        Ok(ack)
    }

    /// Send a message with a specific establishment protocol and wait for the ack
    pub async fn send_waiton_ack_with_establishment(
        &self,
        body: Value,
        protocol: EstablishmentProtocol,
        timeout: Duration,
    ) -> JaResult<JaResponse> {
        let ack = self
            .inner
            .transport
            .send_msg_waiton_ack_with_establishment(HandleMessageWithEstablishmentAndTimeout {
                session_id: self.inner.session_id,
                handle_id: self.inner.id,
                body,
                protocol,
                timeout,
            })
            .await?;
        Ok(ack)
    }

    /// Send a one-shot message with a specific establishment protocol
    pub async fn fire_and_forget_with_establishment(
        &self,
        body: Value,
        protocol: EstablishmentProtocol,
    ) -> JaResult<()> {
        self.inner
            .transport
            .fire_and_forget_msg_with_establishment(HandleMessageWithEstablishment {
                session_id: self.inner.session_id,
                handle_id: self.inner.id,
                body,
                protocol,
            })
            .await?;
        Ok(())
    }
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
