use std::ops::Deref;
use std::time::Duration;

use jarust::prelude::*;
use jarust_rt::JaTask;

use crate::video_room::messages::*;
use crate::video_room::responses::*;

pub struct VideoRoomHandle {
    handle: JaHandle,
    task: Option<JaTask>,
}

impl VideoRoomHandle {

    //
    // synchronous methods
    //

    pub async fn create_room(&self, room: Option<u64>, timeout: Duration) -> JaResult<RoomCreatedRsp> {
        self.create_room_with_config(
            VideoRoomCreateOptions {
                room,
                ..Default::default()
            },
            timeout,
        ).await
    }

    pub async fn create_room_with_config(
        &self,
        options: VideoRoomCreateOptions,
        timeout: Duration,
    ) -> JaResult<RoomCreatedRsp> {
        self
            .handle
            .send_waiton_rsp::<RoomCreatedRsp>(
                serde_json::to_value(VideoRoomCreateMsg::new(options))?,
                timeout,
            ).await
    }

    pub async fn destroy_room(&self, room: u64, options: VideoRoomDestroyOptions, timeout: Duration) -> JaResult<RoomDestroyedRsp> {
        self.handle
            .send_waiton_rsp::<RoomDestroyedRsp>(
                serde_json::to_value(VideoRoomDestroyMsg::new(room, options))?,
                timeout,
            ).await
    }

    pub async fn edit_room(&self, room: u64, options: VideoRoomEditOptions, timeout: Duration) -> JaResult<RoomEditedRsp> {
        self.handle
            .send_waiton_rsp::<RoomEditedRsp>(
                serde_json::to_value(VideoRoomEditMsg::new(room, options))?,
                timeout,
            )
            .await
    }

    pub async fn exists(&self, room: u64, timeout: Duration) -> JaResult<RoomExistsRsp> {
        self
            .handle
            .send_waiton_rsp::<RoomExistsRsp>(
                serde_json::to_value(VideoRoomExistsMsg::new(room)).unwrap(),
                timeout,
            ).await
    }

    pub async fn list(&self, timeout: Duration) -> JaResult<Vec<Room>> {
        let response = self
            .handle
            .send_waiton_rsp::<ListRoomsRsp>(
                serde_json::to_value(VideoRoomListMsg::default())?,
                timeout,
            ).await?;

        Ok(response.list)
    }

    pub async fn allowed(&self, room: u64, action: VideoRoomAllowedAction, allowed: Vec<String>, options: VideoRoomAllowedOptions, timeout: Duration) -> JaResult<AccessRsp> {
        if (action == VideoRoomAllowedAction::Enable || action == VideoRoomAllowedAction::Disable) && !allowed.is_empty() {
            return Err(JaError::InvalidJanusRequest { reason: "An enable or disable 'allowed' request cannot have its allowed array set".to_string() });
        }

        self.handle
            .send_waiton_rsp::<AccessRsp>(
                serde_json::to_value(VideoRoomAllowedMsg::new(room, action, allowed, options)).unwrap(),
                timeout,
            ).await
    }

    pub async fn kick(&self, room: u64, participant: u64, options: VideoRoomKickOptions, timeout: Duration) -> JaResult<()> {
        self
            .handle
            .send_waiton_rsp::<()>(
                serde_json::to_value(VideoRoomKickMsg::new(room, participant, options)).unwrap(),
                timeout,
            ).await
    }

    pub async fn moderate(&self, room: u64, participant: u64, m_line: u64, options: VideoRoomModerateOptions, timeout: Duration) -> JaResult<()> {
        self.handle
            .send_waiton_rsp::<()>(
                serde_json::to_value(VideoRoomModerateMsg::new(room, participant, m_line, options)).unwrap(),
                timeout,
            ).await
    }

    pub async fn enable_recording(&self, room: u64, options: VideoRoomEnableRecordingOptions, timeout: Duration) -> JaResult<()> {
        self.handle
            .send_waiton_rsp::<()>(
                serde_json::to_value(VideoRoomEnableRecordingMsg::new(room, options)).unwrap(),
                timeout,
            ).await
    }

    pub async fn list_participants(&self, room: u64, timeout: Duration) -> JaResult<ListParticipantsRsp> {
        self
            .handle
            .send_waiton_rsp::<ListParticipantsRsp>(
                serde_json::to_value(VideoRoomListParticipantsMsg::new(room)).unwrap(),
                timeout,
            ).await
    }

    pub async fn list_forwarders(&self, room: u64, options: VideoRoomListForwardersOptions, timeout: Duration) -> JaResult<(u64, Vec<RtpForwarderPublisher>)> {
        let response = self
            .handle
            .send_waiton_rsp::<VideoRoomPluginData>(
                serde_json::to_value(VideoRoomListForwardersMsg::new(room, options)).unwrap(),
                timeout,
            ).await?;

        let result = match response.event {
            VideoRoomPluginEvent::ListRtpForward { room, publishers, .. } => (room, publishers),
            _ => {
                return Err(JaError::UnexpectedResponse);
            }
        };

        Ok(result)
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
