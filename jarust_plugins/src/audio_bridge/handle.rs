use super::messages::AudioBridgeAction;
use super::messages::AudioBridgeAllowedMsg;
use super::messages::AudioBridgeAllowedOptions;
use super::messages::AudioBridgeDestroyMsg;
use super::messages::AudioBridgeDestroyOptions;
use super::messages::AudioBridgeEditMsg;
use super::messages::AudioBridgeEditOptions;
use super::messages::AudioBridgeExistsMsg;
use super::messages::AudioBridgeJoinMsg;
use super::messages::AudioBridgeJoinOptions;
use super::messages::AudioBridgeListMsg;
use super::messages::AudioBridgeListParticipantsMsg;
use super::messages::CreateRoomMsg;
use super::responses::AllowedRsp;
use super::responses::ExistsRoomRsp;
use super::responses::ListParticipantsRsp;
use super::responses::ListRoomsRsp;
use super::responses::Room;
use super::responses::RoomCreatedRsp;
use super::responses::RoomDestroyedRsp;
use super::responses::RoomEditedRsp;
use jarust::japrotocol::EstablishmentProtocol;
use jarust::jatask::AbortHandle;
use jarust::prelude::*;
use std::ops::Deref;
use std::time::Duration;

pub struct AudioBridgeHandle {
    handle: JaHandle,
    abort_handles: Option<Vec<AbortHandle>>,
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
            .send_waiton_result::<RoomCreatedRsp>(message, timeout)
            .await
    }

    /// Allows you to dynamically edit some room properties (e.g., the PIN)
    pub async fn edit_room(
        &self,
        room: u64,
        options: AudioBridgeEditOptions,
        timeout: Duration,
    ) -> JaResult<RoomEditedRsp> {
        self.handle
            .send_waiton_result::<RoomEditedRsp>(
                serde_json::to_value(AudioBridgeEditMsg::new(room, options))?,
                timeout,
            )
            .await
    }

    /// Removes an audio conference bridge and destroys it,
    /// kicking all the users out as part of the process
    pub async fn destroy_room(
        &self,
        room: u64,
        options: AudioBridgeDestroyOptions,
        timeout: Duration,
    ) -> JaResult<RoomDestroyedRsp> {
        self.handle
            .send_waiton_result::<RoomDestroyedRsp>(
                serde_json::to_value(AudioBridgeDestroyMsg::new(room, options))?,
                timeout,
            )
            .await
    }

    /// Join an audio room with the given room number and options.
    pub async fn join_room(
        &self,
        room: u64,
        options: AudioBridgeJoinOptions,
        protocol: Option<EstablishmentProtocol>,
        timeout: Duration,
    ) -> JaResult<()> {
        match protocol {
            Some(protocol) => {
                self.handle
                    .send_waiton_ack_with_establishment(
                        serde_json::to_value(AudioBridgeJoinMsg::new(room, options))?,
                        protocol,
                        timeout,
                    )
                    .await?
            }
            None => {
                self.handle
                    .send_waiton_ack(
                        serde_json::to_value(AudioBridgeJoinMsg::new(room, options))?,
                        timeout,
                    )
                    .await?
            }
        };
        Ok(())
    }

    /// Lists all the available rooms.
    pub async fn list_rooms(&self, timeout: Duration) -> JaResult<Vec<Room>> {
        let response = self
            .handle
            .send_waiton_result::<ListRoomsRsp>(
                serde_json::to_value(AudioBridgeListMsg::default())?,
                timeout,
            )
            .await?;
        Ok(response.list)
    }

    /// Allows you to edit who's allowed to join a room via ad-hoc tokens
    pub async fn allowed(
        &self,
        room: u64,
        action: AudioBridgeAction,
        allowed: Vec<String>,
        options: AudioBridgeAllowedOptions,
        timeout: Duration,
    ) -> JaResult<AllowedRsp> {
        self.handle
            .send_waiton_result::<AllowedRsp>(
                serde_json::to_value(AudioBridgeAllowedMsg::new(room, action, allowed, options))?,
                timeout,
            )
            .await
    }

    /// Allows you to check whether a specific audio conference room exists
    pub async fn exists(&self, room: u64, timeout: Duration) -> JaResult<bool> {
        let response = self
            .handle
            .send_waiton_result::<ExistsRoomRsp>(
                serde_json::to_value(AudioBridgeExistsMsg::new(room))?,
                timeout,
            )
            .await?;

        Ok(response.exists)
    }

    /// Lists all the participants of a specific room and their details
    pub async fn list_participants(
        &self,
        room: u64,
        timeout: Duration,
    ) -> JaResult<ListParticipantsRsp> {
        self.handle
            .send_waiton_result::<ListParticipantsRsp>(
                serde_json::to_value(AudioBridgeListParticipantsMsg::new(room))?,
                timeout,
            )
            .await
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
