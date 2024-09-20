use crate::prelude::*;
use jarust_transport::handle_msg::HandleMessage;
use jarust_transport::handle_msg::HandleMessageWithEstablishment;
use jarust_transport::handle_msg::HandleMessageWithEstablishmentAndTimeout;
use jarust_transport::handle_msg::HandleMessageWithTimeout;
use jarust_transport::janus_interface::JanusInterfaceImpl;
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

pub struct NewHandleParams {
    pub handle_id: u64,
    pub session_id: u64,
    pub interface: JanusInterfaceImpl,
}

impl JaHandle {
    pub(crate) async fn new(params: NewHandleParams) -> Self {
        Self {
            inner: Arc::new(InnerHandle {
                id: params.handle_id,
                session_id: params.session_id,
                interface: params.interface,
            }),
        }
    }

    #[inline]
    pub fn id(&self) -> u64 {
        self.inner.id
    }

    #[inline]
    pub fn session_id(&self) -> u64 {
        self.inner.session_id
    }

    /// Send a one-shot message
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn fire_and_forget(&self, body: Value) -> JaResult<()> {
        tracing::debug!("Sending one-shot message");
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
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn send_waiton_rsp<R>(&self, body: Value, timeout: Duration) -> JaResult<R>
    where
        R: DeserializeOwned,
    {
        tracing::debug!("Sending message and waiting for response");
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
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn send_waiton_ack(&self, body: Value, timeout: Duration) -> JaResult<JaResponse> {
        tracing::debug!("Sending message and waiting for ackowledgement");
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
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn send_waiton_ack_with_est(
        &self,
        body: Value,
        protocol: EstablishmentProtocol,
        timeout: Duration,
    ) -> JaResult<JaResponse> {
        tracing::debug!("Sending message with establishment and waiting for ackowledgement");
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
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn fire_and_forget_with_est(
        &self,
        body: Value,
        protocol: EstablishmentProtocol,
    ) -> JaResult<()> {
        tracing::debug!("Sending a one-shot message with establishment");
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
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn hangup(&self, timeout: Duration) -> JaResult<()> {
        tracing::info!("Hanging up");
        let request = json!({
            "janus": "hangup"
        });
        self.send_waiton_ack(request, timeout).await?;
        Ok(())
    }

    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn detach(self, timeout: Duration) -> JaResult<()> {
        tracing::info!("Detaching");
        let request = json!({
            "janus": "detach"
        });
        self.send_waiton_ack(request, timeout).await?;
        Ok(())
    }

    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn trickle_single_candidate(
        &self,
        candidate: Candidate,
        timeout: Duration,
    ) -> JaResult<()> {
        tracing::info!("Trickling single candidate");
        let request = json!({
            "janus": "trickle",
            "candidate": candidate
        });
        self.send_waiton_ack(request, timeout).await?;
        Ok(())
    }

    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn trickle_candidates(
        &self,
        candidates: Vec<Candidate>,
        timeout: Duration,
    ) -> JaResult<()> {
        tracing::info!("Trickling candidates");
        let request = json!({
            "janus": "trickle",
            "candidates": candidates
        });
        self.send_waiton_ack(request, timeout).await?;
        Ok(())
    }

    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn complete_trickle(&self, timeout: Duration) -> JaResult<()> {
        tracing::info!("Completing trickle");
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
