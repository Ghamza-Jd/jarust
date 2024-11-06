use jarust_interface::handle_msg::HandleMessage;
use jarust_interface::handle_msg::HandleMessageWithJsep;
use jarust_interface::janus_interface::JanusInterfaceImpl;
use jarust_interface::japrotocol::Candidate;
use jarust_interface::japrotocol::JaResponse;
use jarust_interface::japrotocol::Jsep;
use serde::de::DeserializeOwned;
use serde_json::json;
use serde_json::Value;
use std::time::Duration;

struct InnerHandle {
    id: u64,
    session_id: u64,
    interface: JanusInterfaceImpl,
}

pub struct JaHandle {
    inner: InnerHandle,
}

pub struct NewHandleParams {
    pub handle_id: u64,
    pub session_id: u64,
    pub interface: JanusInterfaceImpl,
}

impl JaHandle {
    pub(crate) async fn new(params: NewHandleParams) -> Self {
        Self {
            inner: InnerHandle {
                id: params.handle_id,
                session_id: params.session_id,
                interface: params.interface,
            },
        }
    }

    /// Send a one-shot message
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn fire_and_forget(&self, body: Value) -> Result<(), jarust_interface::Error> {
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
    pub async fn send_waiton_rsp<R>(
        &self,
        body: Value,
        timeout: Duration,
    ) -> Result<R, jarust_interface::Error>
    where
        R: DeserializeOwned,
    {
        tracing::debug!("Sending message and waiting for response");
        let res = self
            .inner
            .interface
            .send_msg_waiton_rsp(
                HandleMessage {
                    session_id: self.inner.session_id,
                    handle_id: self.inner.id,
                    body,
                },
                timeout,
            )
            .await?;
        Ok(res)
    }

    /// Send a message and wait for acknowledgement
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn send_waiton_ack(
        &self,
        body: Value,
        timeout: Duration,
    ) -> Result<JaResponse, jarust_interface::Error> {
        tracing::debug!("Sending message and waiting for acknowledgement");
        let ack = self
            .inner
            .interface
            .send_msg_waiton_ack(
                HandleMessage {
                    session_id: self.inner.session_id,
                    handle_id: self.inner.id,
                    body,
                },
                timeout,
            )
            .await?;
        Ok(ack)
    }

    /// Send a message with a jsep and wait for acknowledgement
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn send_waiton_ack_with_jsep(
        &self,
        body: Value,
        jsep: Jsep,
        timeout: Duration,
    ) -> Result<JaResponse, jarust_interface::Error> {
        tracing::debug!("Sending message with jsep and waiting for acknowledgement");
        let ack = self
            .inner
            .interface
            .send_msg_waiton_ack_with_jsep(
                HandleMessageWithJsep {
                    session_id: self.inner.session_id,
                    handle_id: self.inner.id,
                    body,
                    jsep,
                },
                timeout,
            )
            .await?;
        Ok(ack)
    }

    /// Send a one-shot message with a jsep
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn fire_and_forget_with_jsep(
        &self,
        body: Value,
        jsep: Jsep,
    ) -> Result<(), jarust_interface::Error> {
        tracing::debug!("Sending a one-shot message with jsep");
        self.inner
            .interface
            .fire_and_forget_msg_with_jsep(HandleMessageWithJsep {
                session_id: self.inner.session_id,
                handle_id: self.inner.id,
                body,
                jsep,
            })
            .await?;
        Ok(())
    }

    async fn send_handle_request(
        &self,
        body: Value,
        timeout: Duration,
    ) -> Result<JaResponse, jarust_interface::Error> {
        tracing::debug!("Sending a handle request");
        self.inner
            .interface
            .send_handle_request(
                HandleMessage {
                    session_id: self.inner.session_id,
                    handle_id: self.inner.id,
                    body,
                },
                timeout,
            )
            .await
    }
}

impl JaHandle {
    /// Hang up the associated PeerConnection but keep the handle alive
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn hangup(&self, timeout: Duration) -> Result<(), jarust_interface::Error> {
        tracing::info!("Hanging up");
        let request = json!({
            "janus": "hangup"
        });
        self.send_handle_request(request, timeout).await?;
        Ok(())
    }

    /// Destroy the plugin handle
    ///
    /// Similar to [`into_detach`](Self::into_detach) but it borrows the handle instead of consuming it
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn detach(&self, timeout: Duration) -> Result<(), jarust_interface::Error> {
        tracing::info!("Detaching handle");
        let request = json!({
            "janus": "detach"
        });
        self.send_handle_request(request, timeout).await?;
        Ok(())
    }

    /// Destory the plugin handle
    ///
    /// Similar to [`detach`](Self::detach) but consumes the handle
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn into_detach(self, timeout: Duration) -> Result<(), jarust_interface::Error> {
        tracing::info!("Detaching and dropping handle");
        let request = json!({
            "janus": "detach"
        });
        self.send_handle_request(request, timeout).await?;
        Ok(())
    }

    /// Trickles a single ICE candidate to the Janus server
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn trickle_single_candidate(
        &self,
        candidate: Candidate,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!("Trickling single candidate");
        let request = json!({
            "janus": "trickle",
            "candidate": candidate
        });
        self.send_handle_request(request, timeout).await?;
        Ok(())
    }

    /// Trickle multiple ICE candidate to the Janus server
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn trickle_candidates(
        &self,
        candidates: Vec<Candidate>,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!("Trickling candidates");
        let request = json!({
            "janus": "trickle",
            "candidates": candidates
        });
        self.send_handle_request(request, timeout).await?;
        Ok(())
    }

    /// Complete trickle to tell janus server that you sent all the trickle candidates that were gathered.
    ///
    /// This should be send after [`trickle_single_candidate`](Self::trickle_single_candidate) or [`trickle_candidates`](Self::trickle_candidates)
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(session_id = self.inner.session_id, handle_id = self.inner.id))]
    pub async fn complete_trickle(&self, timeout: Duration) -> Result<(), jarust_interface::Error> {
        tracing::info!("Completing trickle");
        let request = json!({
            "janus": "trickle",
            "candidate": {
                "completed" : true
            }
        });
        self.send_handle_request(request, timeout).await?;
        Ok(())
    }
}
