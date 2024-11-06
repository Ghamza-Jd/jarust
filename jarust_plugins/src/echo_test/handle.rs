use super::params::EchoTestStartParams;
use jarust_core::prelude::*;
use jarust_interface::japrotocol::Jsep;
use jarust_rt::JaTask;
use std::ops::Deref;
use std::time::Duration;

pub struct EchoTestHandle {
    handle: JaHandle,
    task: Option<JaTask>,
}

impl EchoTestHandle {
    /// Start/update an echotest session
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn start(&self, params: EchoTestStartParams) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "echotest", "Sending start");
        self.handle.fire_and_forget(params.try_into()?).await
    }

    /// Start/update an echotest session with jsep
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn start_with_jsep(
        &self,
        params: EchoTestStartParams,
        jsep: Jsep,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "echotest", "Sending start with jsep");
        self.send_waiton_ack_with_jsep(params.try_into()?, jsep, timeout)
            .await?;
        Ok(())
    }
}

impl PluginTask for EchoTestHandle {
    fn assign_task(&mut self, task: JaTask) {
        self.task = Some(task);
    }

    fn cancel_task(&mut self) {
        if let Some(task) = self.task.take() {
            task.cancel();
        };
    }
}

impl From<JaHandle> for EchoTestHandle {
    fn from(handle: JaHandle) -> Self {
        Self { handle, task: None }
    }
}

impl Deref for EchoTestHandle {
    type Target = JaHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Drop for EchoTestHandle {
    fn drop(&mut self) {
        self.cancel_task();
    }
}
