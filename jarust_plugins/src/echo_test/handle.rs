use super::msg_options::StartOptions;
use jarust::japrotocol::EstablishmentProtocol;
use jarust::prelude::*;
use jarust_rt::JaTask;
use std::ops::Deref;
use std::time::Duration;

pub struct EchoTestHandle {
    handle: JaHandle,
    task: Option<JaTask>,
}

impl EchoTestHandle {
    pub async fn start(&self, options: StartOptions) -> JaResult<()> {
        self.handle
            .fire_and_forget(serde_json::to_value(options)?)
            .await
    }

    pub async fn start_with_establishment(
        &self,
        options: StartOptions,
        establishment: EstablishmentProtocol,
        timeout: Duration,
    ) -> JaResult<()> {
        self.send_waiton_ack_with_establishment(
            serde_json::to_value(options)?,
            establishment,
            timeout,
        )
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
