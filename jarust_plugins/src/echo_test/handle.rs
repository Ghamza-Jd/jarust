use super::msg_options::EchoTestStartOptions;
use jarust::prelude::*;
use jarust_interface::japrotocol::EstablishmentProtocol;
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
    pub async fn start(&self, options: EchoTestStartOptions) -> jarust_interface::Result<()> {
        tracing::info!(plugin = "echotest", "Sending start");
        self.handle.fire_and_forget(options.try_into()?).await
    }

    /// Start/update an echotest session with establishment
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn start_with_est(
        &self,
        options: EchoTestStartOptions,
        establishment: EstablishmentProtocol,
        timeout: Duration,
    ) -> jarust_interface::Result<()> {
        tracing::info!(plugin = "echotest", "Sending start with establishment");
        self.send_waiton_ack_with_est(options.try_into()?, establishment, timeout)
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

impl Clone for EchoTestHandle {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            task: None,
        }
    }
}
