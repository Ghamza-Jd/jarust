use super::{
    messages::{AudioBridgeCreateMsg, AudioBridgeListMsg},
    results::{AudioBridgePluginData, AudioBridgePluginEvent, Room},
};
use jarust::prelude::*;
use std::ops::Deref;
use tokio::task::AbortHandle;

pub struct AudioBridgeHandle {
    handle: JaHandle,
    abort_handles: Option<Vec<AbortHandle>>,
}

impl AudioBridgeHandle {
    pub async fn create_room(&self, room: Option<u64>) -> JaResult<(u64, bool)> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeCreateMsg::new(
                    room, None, None, None, None, None, None, None, None, None, None, None, None,
                    None, None, None, None, None, None, None, None, None,
                ),
            )?)
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::CreateRoom {
                room, permanent, ..
            } => (room, permanent),
            _ => {
                panic!("Unexpected Response!")
            }
        };

        Ok(result)
    }

    pub async fn create_room_with_config(
        &self,
        room: Option<u64>,
        permanent: Option<bool>,
        description: Option<String>,
        secret: Option<String>,
        pin: Option<String>,
        is_private: Option<bool>,
        allowed: Option<Vec<String>>,
        sampling_rate: Option<u64>,
        spatial_audio: Option<bool>,
        audiolevel_ext: Option<bool>,
        audiolevel_event: Option<bool>,
        audio_active_packets: Option<u64>,
        audio_level_average: Option<u64>,
        default_expectedloss: Option<u64>,
        default_bitrate: Option<u64>,
        record: Option<bool>,
        record_file: Option<String>,
        record_dir: Option<String>,
        mjrs: Option<bool>,
        mjrs_dir: Option<String>,
        allow_rtp_participants: Option<bool>,
        groups: Option<Vec<String>>,
    ) -> JaResult<(u64, bool)> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeCreateMsg::new(
                    room,
                    permanent,
                    description,
                    secret,
                    pin,
                    is_private,
                    allowed,
                    sampling_rate,
                    spatial_audio,
                    audiolevel_ext,
                    audiolevel_event,
                    audio_active_packets,
                    audio_level_average,
                    default_expectedloss,
                    default_bitrate,
                    record,
                    record_file,
                    record_dir,
                    mjrs,
                    mjrs_dir,
                    allow_rtp_participants,
                    groups,
                ),
            )?)
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::CreateRoom {
                room, permanent, ..
            } => (room, permanent),
            _ => {
                panic!("Unexpected Response!")
            }
        };

        Ok(result)
    }

    pub async fn list(&self) -> JaResult<Vec<Room>> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeListMsg::default(),
            )?)
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::List { list, .. } => list,
            _ => {
                panic!("Unexpected Response!")
            }
        };
        Ok(result)
    }
}

impl PluginTask for AudioBridgeHandle {
    fn assign_aborts(&mut self, abort_handles: Vec<AbortHandle>) {
        self.abort_handles = Some(abort_handles);
    }

    fn abort_plugin(&mut self) {
        if let Some(abort_handles) = self.abort_handles.take() {
            for abort_handle in abort_handles {
                abort_handle.abort();
            }
        };
    }
}

impl From<JaHandle> for AudioBridgeHandle {
    fn from(handle: JaHandle) -> Self {
        Self {
            handle,
            abort_handles: None,
        }
    }
}

impl Deref for AudioBridgeHandle {
    type Target = JaHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Drop for AudioBridgeHandle {
    fn drop(&mut self) {
        self.abort_plugin();
    }
}
