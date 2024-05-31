use jarust::prelude::*;
use jarust_rt::JaTask;
use std::ops::Deref;

pub struct VideoRoomHandle {
    handle: JaHandle,
    task: Option<JaTask>,
}

impl PluginTask for VideoRoomHandle {
    fn assign_task(&mut self, task: JaTask) {
        self.task = Some(task);
    }

    fn cancel_task(&mut self) {
        if let Some(task) = self.task.take() {
            task.cancel()
        };
    }
}

impl From<JaHandle> for VideoRoomHandle {
    fn from(handle: JaHandle) -> Self {
        Self { handle, task: None }
    }
}

impl Deref for VideoRoomHandle {
    type Target = JaHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Drop for VideoRoomHandle {
    fn drop(&mut self) {
        self.cancel_task();
    }
}
