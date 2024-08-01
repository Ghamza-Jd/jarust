use std::ops::Deref;
use std::time::Duration;

use serde_json::json;

use jarust::prelude::*;
use jarust_rt::JaTask;

use crate::video_room::msg_options::*;
use crate::video_room::responses::*;

pub struct VideoRoomHandle {
    handle: JaHandle,
    task: Option<JaTask>,
}

//
// synchronous methods
//
impl VideoRoomHandle {
    pub async fn create_room(
        &self,
        room: Option<u64>,
        timeout: Duration,
    ) -> JaResult<RoomCreatedRsp> {
        self.create_room_with_config(
            VideoRoomCreateOptions {
                room,
                ..Default::default()
            },
            timeout,
        )
        .await
    }

    pub async fn create_room_with_config(
        &self,
        options: VideoRoomCreateOptions,
        timeout: Duration,
    ) -> JaResult<RoomCreatedRsp> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "create".into();

        self.handle
            .send_waiton_rsp::<RoomCreatedRsp>(message, timeout)
            .await
    }

    pub async fn destroy_room(
        &self,
        room: u64,
        options: VideoRoomDestroyOptions,
        timeout: Duration,
    ) -> JaResult<RoomDestroyedRsp> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "destroy".into();
        message["room"] = serde_json::to_value(&room)?;

        self.handle
            .send_waiton_rsp::<RoomDestroyedRsp>(message, timeout)
            .await
    }

    pub async fn edit_room(
        &self,
        room: u64,
        options: VideoRoomEditOptions,
        timeout: Duration,
    ) -> JaResult<RoomEditedRsp> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "edit".into();
        message["room"] = serde_json::to_value(&room)?;

        self.handle
            .send_waiton_rsp::<RoomEditedRsp>(message, timeout)
            .await
    }

    pub async fn exists(&self, room: u64, timeout: Duration) -> JaResult<RoomExistsRsp> {
        let message = json!({
            "request": "exists",
            "room": room
        });

        self.handle
            .send_waiton_rsp::<RoomExistsRsp>(message, timeout)
            .await
    }

    pub async fn list(&self, timeout: Duration) -> JaResult<Vec<Room>> {
        let response = self
            .handle
            .send_waiton_rsp::<ListRoomsRsp>(
                json!({
                    "request": "list"
                }),
                timeout,
            )
            .await?;

        Ok(response.list)
    }

    pub async fn allowed(
        &self,
        room: u64,
        action: VideoRoomAllowedAction,
        allowed: Vec<String>,
        options: VideoRoomAllowedOptions,
        timeout: Duration,
    ) -> JaResult<AccessRsp> {
        if (action == VideoRoomAllowedAction::Enable || action == VideoRoomAllowedAction::Disable)
            && !allowed.is_empty()
        {
            return Err(JaError::InvalidJanusRequest {
                reason: "An enable or disable 'allowed' request cannot have its allowed array set"
                    .to_string(),
            });
        }

        let mut message = serde_json::to_value(options)?;
        message["request"] = "allowed".into();
        message["room"] = serde_json::to_value(room)?;
        message["action"] = serde_json::to_value(action)?;
        if !allowed.is_empty() {
            message["allowed"] = serde_json::to_value(allowed)?;
        }

        self.handle
            .send_waiton_rsp::<AccessRsp>(message, timeout)
            .await
    }

    pub async fn kick(
        &self,
        room: u64,
        participant: u64,
        options: VideoRoomKickOptions,
        timeout: Duration,
    ) -> JaResult<()> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "kick".into();
        message["room"] = serde_json::to_value(room)?;
        message["participant"] = serde_json::to_value(participant)?;

        self.handle.send_waiton_rsp::<()>(message, timeout).await
    }

    pub async fn moderate(
        &self,
        room: u64,
        participant: u64,
        m_line: u64,
        options: VideoRoomModerateOptions,
        timeout: Duration,
    ) -> JaResult<()> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "moderate".into();
        message["room"] = serde_json::to_value(&room)?;
        message["participant"] = serde_json::to_value(&participant)?;
        message["m_line"] = serde_json::to_value(&m_line)?;

        self.handle.send_waiton_rsp::<()>(message, timeout).await
    }

    pub async fn enable_recording(
        &self,
        room: u64,
        options: VideoRoomEnableRecordingOptions,
        timeout: Duration,
    ) -> JaResult<()> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "enable_recording".into();
        message["room"] = serde_json::to_value(&room)?;

        self.handle.send_waiton_rsp::<()>(message, timeout).await
    }

    pub async fn list_participants(
        &self,
        room: u64,
        timeout: Duration,
    ) -> JaResult<ListParticipantsRsp> {
        self.handle
            .send_waiton_rsp::<ListParticipantsRsp>(
                json!({
                    "request": "list_participants",
                    "room": room
                }),
                timeout,
            )
            .await
    }

    pub async fn list_forwarders(
        &self,
        room: u64,
        options: VideoRoomListForwardersOptions,
        timeout: Duration,
    ) -> JaResult<ListForwardersRsp> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "list_forwarders".into();
        message["room"] = serde_json::to_value(&room)?;

        self.handle
            .send_waiton_rsp::<ListForwardersRsp>(message, timeout)
            .await
    }
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
