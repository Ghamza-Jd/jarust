use super::events::PluginEvent;
use super::handle::VideoRoomHandle;
use jarust::prelude::*;
use std::ops::Deref;
use std::time::Duration;
use tokio::sync::mpsc;

#[async_trait::async_trait]
pub trait VideoRoom: Attach {
    type Event: TryFrom<JaResponse, Error = JaError> + Send + Sync + 'static;
    type Handle: From<JaHandle> + Deref<Target = JaHandle> + PluginTask;

    async fn attach_video_room(
        &self,
        capacity: usize,
        timeout: Duration,
    ) -> JaResult<(Self::Handle, mpsc::UnboundedReceiver<Self::Event>)> {
        let (handle, mut receiver) = self
            .attach("janus.plugin.videoroom", capacity, timeout)
            .await?;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let task = jarust_rt::spawn(async move {
            while let Some(rsp) = receiver.recv().await {
                if let Ok(event) = rsp.try_into() {
                    let _ = tx.send(event);
                };
            }
        });
        let mut handle: Self::Handle = handle.into();
        handle.assign_task(task);
        Ok((handle, rx))
    }
}

impl VideoRoom for JaSession {
    type Event = PluginEvent;
    type Handle = VideoRoomHandle;
}
