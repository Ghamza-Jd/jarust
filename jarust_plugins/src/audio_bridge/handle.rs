use super::messages::AllowedMsg;
use super::messages::CreateRoomMsg;
use super::messages::DestroyRoomMsg;
use super::messages::EditRoomMsg;
use super::messages::JoinRoomMsg;
use super::responses::AllowedRsp;
use super::responses::ExistsRoomRsp;
use super::responses::ListParticipantsRsp;
use super::responses::ListRoomsRsp;
use super::responses::Room;
use super::responses::RoomCreatedRsp;
use super::responses::RoomDestroyedRsp;
use super::responses::RoomEditedRsp;
use jarust::japrotocol::EstablishmentProtocol;
use jarust::prelude::*;
use jarust_rt::JaTask;
use serde_json::json;
use std::ops::Deref;
use std::time::Duration;

pub struct AudioBridgeHandle {
    handle: JaHandle,
    task: Option<JaTask>,
}

impl AudioBridgeHandle {
    /// Create a new audio room dynamically with the given room number,
    /// as an alternative to using the configuration file
    ///
    /// Random room number will be used if `room` is `None`
    pub async fn create_room(
        &self,
        room: Option<u64>,
        timeout: Duration,
    ) -> JaResult<RoomCreatedRsp> {
        self.create_room_with_config(
            CreateRoomMsg {
                room,
                ..Default::default()
            },
            timeout,
        )
        .await
    }

    /// Create a new audio room dynamically with the given configuration,
    /// as an alternative to using the configuration file
    ///
    /// Random room number will be used if `room` is `None`
    pub async fn create_room_with_config(
        &self,
        options: CreateRoomMsg,
        timeout: Duration,
    ) -> JaResult<RoomCreatedRsp> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "create".into();
        self.handle
            .send_waiton_rsp::<RoomCreatedRsp>(message, timeout)
            .await
    }

    /// Allows you to dynamically edit some room properties (e.g., the PIN)
    pub async fn edit_room(
        &self,
        room: u64,
        options: EditRoomMsg,
        timeout: Duration,
    ) -> JaResult<RoomEditedRsp> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "edit".into();
        message["room"] = room.into();
        self.handle
            .send_waiton_rsp::<RoomEditedRsp>(message, timeout)
            .await
    }

    /// Removes an audio conference bridge and destroys it,
    /// kicking all the users out as part of the process
    pub async fn destroy_room(
        &self,
        room: u64,
        options: DestroyRoomMsg,
        timeout: Duration,
    ) -> JaResult<RoomDestroyedRsp> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "destroy".into();
        message["room"] = room.into();
        self.handle
            .send_waiton_rsp::<RoomDestroyedRsp>(message, timeout)
            .await
    }

    /// Join an audio room with the given room number and options.
    pub async fn join_room(
        &self,
        room: u64,
        options: JoinRoomMsg,
        protocol: Option<EstablishmentProtocol>,
        timeout: Duration,
    ) -> JaResult<()> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "join".into();
        message["room"] = room.into();
        match protocol {
            Some(protocol) => {
                self.handle
                    .send_waiton_ack_with_establishment(message, protocol, timeout)
                    .await?
            }
            None => self.handle.send_waiton_ack(message, timeout).await?,
        };
        Ok(())
    }

    /// Lists all the available rooms.
    pub async fn list_rooms(&self, timeout: Duration) -> JaResult<Vec<Room>> {
        let message = json!({
            "request": "list"
        });
        let response = self
            .handle
            .send_waiton_rsp::<ListRoomsRsp>(message, timeout)
            .await?;
        Ok(response.list)
    }

    /// Allows you to edit who's allowed to join a room via ad-hoc tokens
    pub async fn allowed(
        &self,
        room: u64,
        options: AllowedMsg,
        timeout: Duration,
    ) -> JaResult<AllowedRsp> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "allowed".into();
        message["room"] = room.into();
        self.handle
            .send_waiton_rsp::<AllowedRsp>(message, timeout)
            .await
    }

    /// Allows you to check whether a specific audio conference room exists
    pub async fn exists(&self, room: u64, timeout: Duration) -> JaResult<bool> {
        let message = json!({
            "request": "exists",
            "room": room
        });
        let response = self
            .handle
            .send_waiton_rsp::<ExistsRoomRsp>(message, timeout)
            .await?;

        Ok(response.exists)
    }

    /// Lists all the participants of a specific room and their details
    pub async fn list_participants(
        &self,
        room: u64,
        timeout: Duration,
    ) -> JaResult<ListParticipantsRsp> {
        let message = json!({
            "request": "listparticipants",
            "room": room
        });
        self.handle
            .send_waiton_rsp::<ListParticipantsRsp>(message, timeout)
            .await
    }
}

impl PluginTask for AudioBridgeHandle {
    fn assign_task(&mut self, task: JaTask) {
        self.task = Some(task);
    }

    fn cancel_task(&mut self) {
        if let Some(task) = self.task.take() {
            task.cancel()
        };
    }
}

impl From<JaHandle> for AudioBridgeHandle {
    fn from(handle: JaHandle) -> Self {
        Self { handle, task: None }
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
        self.cancel_task();
    }
}
