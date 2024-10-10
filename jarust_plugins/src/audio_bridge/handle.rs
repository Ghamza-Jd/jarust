use super::msg_options::AudioBridgeAllowedOptions;
use super::msg_options::AudioBridgeChangeRoomOptions;
use super::msg_options::AudioBridgeConfigureOptions;
use super::msg_options::AudioBridgeCreateRoomOptions;
use super::msg_options::AudioBridgeDestroyRoomMsg;
use super::msg_options::AudioBridgeEditRoomOptions;
use super::msg_options::AudioBridgeJoinRoomOptions;
use super::msg_options::AudioBridgeKickAllOptions;
use super::msg_options::AudioBridgeKickOptions;
use super::msg_options::AudioBridgeMuteOptions;
use super::msg_options::AudioBridgeMuteRoomOptions;
use super::responses::AudioBridgeAllowedRsp;
use super::responses::AudioBridgeExistsRoomRsp;
use super::responses::AudioBridgeListParticipantsRsp;
use super::responses::AudioBridgeListRoomsRsp;
use super::responses::AudioBridgeRoom;
use super::responses::AudioBridgeRoomCreatedRsp;
use super::responses::AudioBridgeRoomDestroyedRsp;
use super::responses::AudioBridgeRoomEditedRsp;
use crate::JanusId;
use jarust::prelude::*;
use jarust_interface::japrotocol::EstablishmentProtocol;
use jarust_rt::JaTask;
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
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn create_room(
        &self,
        room: Option<JanusId>,
        timeout: Duration,
    ) -> Result<AudioBridgeRoomCreatedRsp, jarust_interface::Error> {
        self.create_room_with_config(
            AudioBridgeCreateRoomOptions {
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
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn create_room_with_config(
        &self,
        options: AudioBridgeCreateRoomOptions,
        timeout: Duration,
    ) -> Result<AudioBridgeRoomCreatedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending create room");
        let mut message: Value = options.try_into()?;
        message["request"] = "create".into();
        self.handle
            .send_waiton_rsp::<AudioBridgeRoomCreatedRsp>(message, timeout)
            .await
    }

    /// Allows you to dynamically edit some room properties (e.g., the PIN)
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn edit_room(
        &self,
        room: JanusId,
        options: AudioBridgeEditRoomOptions,
        timeout: Duration,
    ) -> Result<AudioBridgeRoomEditedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending edit room");
        let mut message: Value = options.try_into()?;
        message["request"] = "edit".into();
        message["room"] = room.try_into()?;
        self.handle
            .send_waiton_rsp::<AudioBridgeRoomEditedRsp>(message, timeout)
            .await
    }

    /// Removes an audio conference bridge and destroys it,
    /// kicking all the users out as part of the process
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn destroy_room(
        &self,
        room: JanusId,
        options: AudioBridgeDestroyRoomMsg,
        timeout: Duration,
    ) -> Result<AudioBridgeRoomDestroyedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending destroy room");
        let mut message: Value = options.try_into()?;
        message["request"] = "destroy".into();
        message["room"] = room.try_into()?;
        self.handle
            .send_waiton_rsp::<AudioBridgeRoomDestroyedRsp>(message, timeout)
            .await
    }

    /// Join an audio room with the given room number and options.
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn join_room(
        &self,
        room: JanusId,
        options: AudioBridgeJoinRoomOptions,
        protocol: Option<EstablishmentProtocol>,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending join room");
        let mut message: Value = options.try_into()?;
        message["request"] = "join".into();
        message["room"] = room.try_into()?;
        match protocol {
            Some(protocol) => {
                self.handle
                    .send_waiton_ack_with_est(message, protocol, timeout)
                    .await?
            }
            None => self.handle.send_waiton_ack(message, timeout).await?,
        };
        Ok(())
    }

    /// Lists all the available rooms.
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn list_rooms(
        &self,
        timeout: Duration,
    ) -> Result<Vec<AudioBridgeRoom>, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending list rooms");
        let message = json!({
            "request": "list"
        });
        let response = self
            .handle
            .send_waiton_rsp::<AudioBridgeListRoomsRsp>(message, timeout)
            .await?;
        Ok(response.list)
    }

    /// Allows you to edit who's allowed to join a room via ad-hoc tokens
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn allowed(
        &self,
        room: JanusId,
        options: AudioBridgeAllowedOptions,
        timeout: Duration,
    ) -> Result<AudioBridgeAllowedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending allowed");
        let mut message: Value = options.try_into()?;
        message["request"] = "allowed".into();
        message["room"] = room.try_into()?;
        self.handle
            .send_waiton_rsp::<AudioBridgeAllowedRsp>(message, timeout)
            .await
    }

    /// Allows you to check whether a specific audio conference room exists
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn exists(
        &self,
        room: JanusId,
        timeout: Duration,
    ) -> Result<bool, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending exists");
        let message = json!({
            "request": "exists",
            "room": room
        });
        let response = self
            .handle
            .send_waiton_rsp::<AudioBridgeExistsRoomRsp>(message, timeout)
            .await?;

        Ok(response.exists)
    }

    /// Lists all the participants of a specific room and their details
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn list_participants(
        &self,
        room: JanusId,
        timeout: Duration,
    ) -> Result<AudioBridgeListParticipantsRsp, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending list participants");
        let message = json!({
            "request": "listparticipants",
            "room": room
        });
        self.handle
            .send_waiton_rsp::<AudioBridgeListParticipantsRsp>(message, timeout)
            .await
    }

    /// Configure the media related settings of the participant
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn configure(
        &self,
        options: AudioBridgeConfigureOptions,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending configure");
        let mut message: Value = options.try_into()?;
        message["request"] = "configure".into();
        self.handle.send_waiton_ack(message, timeout).await?;
        Ok(())
    }

    /// Mute a participant
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn mute(
        &self,
        options: AudioBridgeMuteOptions,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending mute");
        let mut message: Value = options.try_into()?;
        message["request"] = "mute".into();
        self.handle.fire_and_forget(message).await
    }

    /// Unmute a participant
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn unmute(
        &self,
        options: AudioBridgeMuteOptions,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending unmute");
        let mut message: Value = options.try_into()?;
        message["request"] = "unmute".into();
        self.handle.fire_and_forget(message).await
    }

    /// Mute a room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn mute_room(
        &self,
        options: AudioBridgeMuteRoomOptions,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending mute room");
        let mut message: Value = options.try_into()?;
        message["request"] = "mute_room".into();
        self.handle.fire_and_forget(message).await
    }

    /// Unmute a room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn unmute_room(
        &self,
        options: AudioBridgeMuteRoomOptions,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending unmute room");
        let mut message: Value = options.try_into()?;
        message["request"] = "unmute_room".into();
        self.handle.fire_and_forget(message).await
    }

    /// Kicks a participants out of a room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn kick(
        &self,
        options: AudioBridgeKickOptions,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending kick");
        let mut message: Value = options.try_into()?;
        message["request"] = "kick".into();
        self.handle.fire_and_forget(message).await
    }

    /// Kicks all participants out of a room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn kick_all(
        &self,
        options: AudioBridgeKickAllOptions,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending kick all");
        let mut message: Value = options.try_into()?;
        message["request"] = "kick_all".into();
        self.handle.fire_and_forget(message).await
    }

    /// Leave an audio room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn leave(&self, timeout: Duration) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending leave");
        let message = json!({
            "request" : "leave"
        });
        self.handle.send_waiton_ack(message, timeout).await?;
        Ok(())
    }

    /// Change the room you are in, instead of leaving and joining a new room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn change_room(
        &self,
        room: JanusId,
        options: AudioBridgeChangeRoomOptions,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending change room");
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
