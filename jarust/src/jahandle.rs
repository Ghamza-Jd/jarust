use crate::prelude::*;
use jarust_transport::handle_msg::HandleMessage;
use jarust_transport::handle_msg::HandleMessageWithEstablishment;
use jarust_transport::handle_msg::HandleMessageWithEstablishmentAndTimeout;
use jarust_transport::handle_msg::HandleMessageWithTimeout;
use jarust_transport::interface::janus_interface::JanusInterfaceImpl;
use jarust_transport::japrotocol::Candidate;
use jarust_transport::japrotocol::EstablishmentProtocol;
use jarust_transport::japrotocol::JaResponse;
use serde::de::DeserializeOwned;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;

struct InnerHandle {
    id: u64,
    session_id: u64,
    interface: JanusInterfaceImpl,
}

#[derive(Clone)]
pub struct JaHandle {
    inner: Arc<InnerHandle>,
}

impl JaHandle {
    pub(crate) async fn new(id: u64, session_id: u64, interface: JanusInterfaceImpl) -> Self {
        Self {
            inner: Arc::new(InnerHandle {
                id,
                session_id,
                interface,
            }),
        }
    }

    /// Send a one-shot message
    pub async fn fire_and_forget(&self, body: Value) -> JaResult<()> {
        self.inner
            .interface
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
            .interface
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
            .interface
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
            .interface
            .send_msg_waiton_ack_with_est(HandleMessageWithEstablishmentAndTimeout {
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
            .interface
            .fire_and_forget_msg_with_est(HandleMessageWithEstablishment {
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
