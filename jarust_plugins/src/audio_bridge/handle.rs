use super::msg_opitons::AllowedOptions;
use super::msg_opitons::ChangeRoomOptions;
use super::msg_opitons::ConfigureOptions;
use super::msg_opitons::CreateRoomOptions;
use super::msg_opitons::DestroyRoomMsg;
use super::msg_opitons::EditRoomOptions;
use super::msg_opitons::JoinRoomOptions;
use super::msg_opitons::KickAllOptions;
use super::msg_opitons::KickOptions;
use super::msg_opitons::MuteOptions;
use super::msg_opitons::MuteRoomOptions;
use super::responses::AllowedRsp;
use super::responses::ExistsRoomRsp;
use super::responses::ListParticipantsRsp;
use super::responses::ListRoomsRsp;
use super::responses::Room;
use super::responses::RoomCreatedRsp;
use super::responses::RoomDestroyedRsp;
use super::responses::RoomEditedRsp;
use crate::Identifier;
use jarust::prelude::*;
use jarust_rt::JaTask;
use jarust_transport_next::japrotocol::EstablishmentProtocol;
use serde_json::json;
use serde_json::Value;
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
        room: Option<Identifier>,
        timeout: Duration,
    ) -> JaResult<RoomCreatedRsp> {
        self.create_room_with_config(
            CreateRoomOptions {
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
        options: CreateRoomOptions,
        timeout: Duration,
    ) -> JaResult<RoomCreatedRsp> {
        let mut message: Value = options.try_into()?;
        message["request"] = "create".into();
        self.handle
            .send_waiton_rsp::<RoomCreatedRsp>(message, timeout)
            .await
    }

    /// Allows you to dynamically edit some room properties (e.g., the PIN)
    pub async fn edit_room(
        &self,
        room: Identifier,
        options: EditRoomOptions,
        timeout: Duration,
    ) -> JaResult<RoomEditedRsp> {
        let mut message: Value = options.try_into()?;
        message["request"] = "edit".into();
        message["room"] = room.try_into()?;
        self.handle
            .send_waiton_rsp::<RoomEditedRsp>(message, timeout)
            .await
    }

    /// Removes an audio conference bridge and destroys it,
    /// kicking all the users out as part of the process
    pub async fn destroy_room(
        &self,
        room: Identifier,
        options: DestroyRoomMsg,
        timeout: Duration,
    ) -> JaResult<RoomDestroyedRsp> {
        let mut message: Value = options.try_into()?;
        message["request"] = "destroy".into();
        message["room"] = room.try_into()?;
        self.handle
            .send_waiton_rsp::<RoomDestroyedRsp>(message, timeout)
            .await
    }

    /// Join an audio room with the given room number and options.
    pub async fn join_room(
        &self,
        room: Identifier,
        options: JoinRoomOptions,
        protocol: Option<EstablishmentProtocol>,
        timeout: Duration,
    ) -> JaResult<()> {
        let mut message: Value = options.try_into()?;
        message["request"] = "join".into();
        message["room"] = room.try_into()?;
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
        room: Identifier,
        options: AllowedOptions,
        timeout: Duration,
    ) -> JaResult<AllowedRsp> {
        let mut message: Value = options.try_into()?;
        message["request"] = "allowed".into();
        message["room"] = room.try_into()?;
        self.handle
            .send_waiton_rsp::<AllowedRsp>(message, timeout)
            .await
    }

    /// Allows you to check whether a specific audio conference room exists
    pub async fn exists(&self, room: Identifier, timeout: Duration) -> JaResult<bool> {
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
        room: Identifier,
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

    /// Configure the media related settings of the participant
    pub async fn configure(&self, options: ConfigureOptions, timeout: Duration) -> JaResult<()> {
        let mut message: Value = options.try_into()?;
        message["request"] = "configure".into();
        self.handle.send_waiton_ack(message, timeout).await?;
        Ok(())
    }

    /// Mute a participant
    pub async fn mute(&self, options: MuteOptions) -> JaResult<()> {
        let mut message: Value = options.try_into()?;
        message["request"] = "mute".into();
        self.handle.fire_and_forget(message).await
    }

    /// Unmute a participant
    pub async fn unmute(&self, options: MuteOptions) -> JaResult<()> {
        let mut message: Value = options.try_into()?;
        message["request"] = "unmute".into();
        self.handle.fire_and_forget(message).await
    }

    /// Mute a room
    pub async fn mute_room(&self, options: MuteRoomOptions) -> JaResult<()> {
        let mut message: Value = options.try_into()?;
        message["request"] = "mute_room".into();
        self.handle.fire_and_forget(message).await
    }

    /// Unmute a room
    pub async fn unmute_room(&self, options: MuteRoomOptions) -> JaResult<()> {
        let mut message: Value = options.try_into()?;
        message["request"] = "unmute_room".into();
        self.handle.fire_and_forget(message).await
    }

    /// Kicks a participants out of a room
    pub async fn kick(&self, options: KickOptions) -> JaResult<()> {
        let mut message: Value = options.try_into()?;
        message["request"] = "kick".into();
        self.handle.fire_and_forget(message).await
    }

    /// Kicks all pariticpants out of a room
    pub async fn kick_all(&self, options: KickAllOptions) -> JaResult<()> {
        let mut message: Value = options.try_into()?;
        message["request"] = "kick_all".into();
        self.handle.fire_and_forget(message).await
    }

    /// Leave an audio room
    pub async fn leave(&self, timeout: Duration) -> JaResult<()> {
        let message = json!({
            "request" : "leave"
        });
        self.handle.send_waiton_ack(message, timeout).await?;
        Ok(())
    }

    /// Change the room you are in, instead of leaving and joining a new room
    pub async fn change_room(
        &self,
        room: Identifier,
        options: ChangeRoomOptions,
        timeout: Duration,
    ) -> JaResult<()> {
        let mut message: Value = options.try_into()?;
        message["request"] = "changeroom".into();
        message["room"] = room.try_into()?;
        self.handle.send_waiton_ack(message, timeout).await?;
        Ok(())
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
