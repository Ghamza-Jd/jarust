use super::params::*;
use super::responses::*;
use crate::JanusId;
use jarust_core::prelude::*;
use jarust_interface::japrotocol::Jsep;
use jarust_rt::JaTask;
use serde_json::json;
use serde_json::Value;
use std::ops::Deref;
use std::time::Duration;

pub struct AudioBridgeHandle {
    handle: JaHandle,
    task: Option<JaTask>,
}

// sync
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
            AudioBridgeCreateParams {
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
        params: AudioBridgeCreateParams,
        timeout: Duration,
    ) -> Result<AudioBridgeRoomCreatedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending create room");
        let mut message: Value = params.try_into()?;
        message["request"] = "create".into();
        self.handle
            .send_waiton_rsp::<AudioBridgeRoomCreatedRsp>(message, timeout)
            .await
    }

    /// Allows you to dynamically edit some room properties (e.g., the PIN)
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn edit_room(
        &self,
        params: AudioBridgeEditParams,
        timeout: Duration,
    ) -> Result<AudioBridgeRoomEditedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending edit room");
        let mut message: Value = params.try_into()?;
        message["request"] = "edit".into();
        self.handle
            .send_waiton_rsp::<AudioBridgeRoomEditedRsp>(message, timeout)
            .await
    }

    /// Removes an audio conference bridge and destroys it,
    /// kicking all the users out as part of the process
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn destroy_room(
        &self,
        params: AudioBridgeDestroyParams,
        timeout: Duration,
    ) -> Result<AudioBridgeRoomDestroyedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending destroy room");
        let mut message: Value = params.try_into()?;
        message["request"] = "destroy".into();
        self.handle
            .send_waiton_rsp::<AudioBridgeRoomDestroyedRsp>(message, timeout)
            .await
    }

    #[cfg(feature = "__experimental")]
    /// To enable or disable recording of mixed audio stream while the conference is in progress
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn enable_recording(
        &self,
        params: AudioBridgeEnableRecordingParams,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending enable recording");
        let mut message: Value = params.try_into()?;
        message["request"] = "enable_recording".into();
        self.handle.send_waiton_ack(message, timeout).await?;
        Ok(())
    }

    /// A room can also be recorded by saving the individual contributions of participants to separate MJR files instead,
    /// in a format compatible with the [Recordings](https://janus.conf.meetecho.com/docs/recordings.html).
    /// While a recording for each participant can be enabled or disabled separately, there also is a request
    /// to enable or disable them in bulk, thus implementing a feature similar to enable_recording but for MJR
    /// files, rather than for a .wav mix
    #[cfg(feature = "__experimental")]
    pub async fn enable_mjrs(
        &self,
        params: AudioBridgeEnableMjrsParams,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending enable mjrs");
        let mut message: Value = params.try_into()?;
        message["request"] = "enable_mjrs".into();
        self.handle.send_waiton_ack(message, timeout).await?;
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
    #[cfg(feature = "__experimental")]
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn allowed(
        &self,
        params: AudioBridgeAllowedParams,
        timeout: Duration,
    ) -> Result<AudioBridgeAllowedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending allowed");
        let mut message: Value = params.try_into()?;
        message["request"] = "allowed".into();
        self.handle
            .send_waiton_rsp::<AudioBridgeAllowedRsp>(message, timeout)
            .await
    }

    /// Allows you to check whether a specific audio conference room exists
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn exists(
        &self,
        params: AudioBridgeExistsParams,
        timeout: Duration,
    ) -> Result<bool, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending exists");
        let mut message: Value = params.try_into()?;
        message["request"] = "exists".into();
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
        params: AudioBridgeListParticipantsParams,
        timeout: Duration,
    ) -> Result<AudioBridgeListParticipantsRsp, jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending list participants");
        let mut message: Value = params.try_into()?;
        message["request"] = "listparticipants".into();
        self.handle
            .send_waiton_rsp::<AudioBridgeListParticipantsRsp>(message, timeout)
            .await
    }

    /// Kicks a participants out of a room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn kick(&self, params: AudioBridgeKickParams) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending kick");
        let mut message: Value = params.try_into()?;
        message["request"] = "kick".into();
        self.handle.fire_and_forget(message).await
    }

    /// Kicks all participants out of a room
    #[cfg(feature = "__experimental")]
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn kick_all(
        &self,
        params: AudioBridgeKickAllParams,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending kick all");
        let mut message: Value = params.try_into()?;
        message["request"] = "kick_all".into();
        self.handle.fire_and_forget(message).await
    }
}

// async
impl AudioBridgeHandle {
    /// Join an audio room with the given room number and options.
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn join_room(
        &self,
        params: AudioBridgeJoinParams,
        jsep: Option<Jsep>,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending join room");
        let mut message: Value = params.try_into()?;
        message["request"] = "join".into();
        match jsep {
            Some(protocol) => {
                self.handle
                    .send_waiton_ack_with_jsep(message, protocol, timeout)
                    .await?
            }
            None => self.handle.send_waiton_ack(message, timeout).await?,
        };
        Ok(())
    }

    /// Configure the media related settings of the participant
    #[cfg(feature = "__experimental")]
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn configure(
        &self,
        params: AudioBridgeConfigureParams,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending configure");
        let mut message: Value = params.try_into()?;
        message["request"] = "configure".into();
        self.handle.send_waiton_ack(message, timeout).await?;
        Ok(())
    }

    /// Mute a participant
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn mute(&self, params: AudioBridgeMuteParams) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending mute");
        let mut message: Value = params.try_into()?;
        message["request"] = "mute".into();
        self.handle.fire_and_forget(message).await
    }

    /// Unmute a participant
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn unmute(
        &self,
        params: AudioBridgeMuteParams,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending unmute");
        let mut message: Value = params.try_into()?;
        message["request"] = "unmute".into();
        self.handle.fire_and_forget(message).await
    }

    /// Mute a room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn mute_room(
        &self,
        params: AudioBridgeMuteRoomParams,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending mute room");
        let mut message: Value = params.try_into()?;
        message["request"] = "mute_room".into();
        self.handle.fire_and_forget(message).await
    }

    /// Unmute a room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn unmute_room(
        &self,
        params: AudioBridgeMuteRoomParams,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending unmute room");
        let mut message: Value = params.try_into()?;
        message["request"] = "unmute_room".into();
        self.handle.fire_and_forget(message).await
    }

    /// Change the room you are in, instead of leaving and joining a new room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn change_room(
        &self,
        params: AudioBridgeChangeRoomParams,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "audiobridge", "Sending change room");
        let mut message: Value = params.try_into()?;
        message["request"] = "changeroom".into();
        self.handle.send_waiton_ack(message, timeout).await?;
        Ok(())
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
