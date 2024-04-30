use super::messages::AudioBridgeAction;
use super::messages::AudioBridgeAllowedMsg;
use super::messages::AudioBridgeAllowedOptions;
use super::messages::AudioBridgeCreateMsg;
use super::messages::AudioBridgeCreateOptions;
use super::messages::AudioBridgeDestroyMsg;
use super::messages::AudioBridgeDestroyOptions;
use super::messages::AudioBridgeEditMsg;
use super::messages::AudioBridgeEditOptions;
use super::messages::AudioBridgeExistsMsg;
use super::messages::AudioBridgeJoinMsg;
use super::messages::AudioBridgeJoinOptions;
use super::messages::AudioBridgeKickAllMsg;
use super::messages::AudioBridgeKickAllOptions;
use super::messages::AudioBridgeKickMsg;
use super::messages::AudioBridgeKickOptions;
use super::messages::AudioBridgeListMsg;
use super::messages::AudioBridgeListParticipantsMsg;
use super::messages::AudioBridgeResumeMsg;
use super::messages::AudioBridgeResumeOptions;
use super::messages::AudioBridgeSuspendMsg;
use super::messages::AudioBridgeSuspendOptions;
use super::responses::AudioBridgePluginData;
use super::responses::AudioBridgePluginEvent;
use super::responses::Participant;
use super::responses::Room;
use super::responses::RoomCreated;
use super::responses::RoomEdited;
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
    pub async fn create_room(&self, room: Option<u64>, timeout: Duration) -> JaResult<RoomCreated> {
        self.create_room_with_config(
            AudioBridgeCreateOptions {
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
        options: AudioBridgeCreateOptions,
        timeout: Duration,
    ) -> JaResult<RoomCreated> {
        self.handle
            .send_waiton_result::<RoomCreated>(
                serde_json::to_value(AudioBridgeCreateMsg::new(options))?,
                timeout,
            )
            .await
    }

    /// Allows you to dynamically edit some room properties (e.g., the PIN)
    pub async fn edit_room(
        &self,
        room: u64,
        options: AudioBridgeEditOptions,
        timeout: Duration,
    ) -> JaResult<RoomEdited> {
        self.handle
            .send_waiton_result::<RoomEdited>(
                serde_json::to_value(AudioBridgeEditMsg::new(room, options))?,
                timeout,
            )
            .await
    }

    /// Eemoves an audio conference bridge and destroys it,
    /// kicking all the users out as part of the process
    pub async fn destroy_room(
        &self,
        room: u64,
        options: AudioBridgeDestroyOptions,
        timeout: Duration,
    ) -> JaResult<(u64, bool)> {
        let response = self
            .handle
            .send_waiton_result::<AudioBridgePluginData>(
                serde_json::to_value(AudioBridgeDestroyMsg::new(room, options))?,
                timeout,
            )
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::DestroyRoom {
                room, permanent, ..
            } => (room, permanent),
            _ => {
                return Err(JaError::UnexpectedResponse);
            }
        };

        Ok(result)
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
    pub async fn list(&self, timeout: Duration) -> JaResult<Vec<Room>> {
        let response = self
            .handle
            .send_waiton_result::<AudioBridgePluginData>(
                serde_json::to_value(AudioBridgeListMsg::default())?,
                timeout,
            )
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::List { list, .. } => list,
            _ => {
                return Err(JaError::UnexpectedResponse);
            }
        };
        Ok(result)
    }

    /// Allows you to edit who's allowed to join a room via ad-hoc tokens
    pub async fn allowed(
        &self,
        room: u64,
        action: AudioBridgeAction,
        allowed: Vec<String>,
        options: AudioBridgeAllowedOptions,
        timeout: Duration,
    ) -> JaResult<(u64, Vec<String>)> {
        let response = self
            .handle
            .send_waiton_result::<AudioBridgePluginData>(
                serde_json::to_value(AudioBridgeAllowedMsg::new(room, action, allowed, options))?,
                timeout,
            )
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::Allowed { room, allowed, .. } => (room, allowed),
            _ => {
                return Err(JaError::UnexpectedResponse);
            }
        };
        Ok(result)
    }

    /// Allows you to check whether a specific audio conference room exists
    pub async fn exists(&self, room: u64, timeout: Duration) -> JaResult<bool> {
        let response = self
            .handle
            .send_waiton_result::<AudioBridgePluginData>(
                serde_json::to_value(AudioBridgeExistsMsg::new(room))?,
                timeout,
            )
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::ExistsRoom { exists, .. } => exists,
            _ => {
                return Err(JaError::UnexpectedResponse);
            }
        };
        Ok(result)
    }

    /// Allows you to kick a participant out of a specific room
    pub async fn kick(
        &self,
        room: u64,
        participant: u64,
        options: AudioBridgeKickOptions,
        timeout: Duration,
    ) -> JaResult<()> {
        let response = self
            .handle
            .send_waiton_result::<AudioBridgePluginData>(
                serde_json::to_value(AudioBridgeKickMsg::new(room, participant, options))?,
                timeout,
            )
            .await?;
        match response.event {
            AudioBridgePluginEvent::Success {} => Ok(()),
            _ => Err(JaError::UnexpectedResponse),
        }
    }

    /// Allows you to kick all participants out of a specific room
    pub async fn kick_all(
        &self,
        room: u64,
        options: AudioBridgeKickAllOptions,
        timeout: Duration,
    ) -> JaResult<()> {
        let response = self
            .handle
            .send_waiton_result::<AudioBridgePluginData>(
                serde_json::to_value(AudioBridgeKickAllMsg::new(room, options))?,
                timeout,
            )
            .await?;
        match response.event {
            AudioBridgePluginEvent::Success {} => Ok(()),
            _ => Err(JaError::UnexpectedResponse),
        }
    }

    /// Allows you to suspend a participant in a specific room
    pub async fn suspend(
        &self,
        room: u64,
        participant: u64,
        options: AudioBridgeSuspendOptions,
        timeout: Duration,
    ) -> JaResult<()> {
        let response = self
            .handle
            .send_waiton_result::<AudioBridgePluginData>(
                serde_json::to_value(AudioBridgeSuspendMsg::new(room, participant, options))?,
                timeout,
            )
            .await?;
        match response.event {
            AudioBridgePluginEvent::Success {} => Ok(()),
            _ => Err(JaError::UnexpectedResponse),
        }
    }

    /// Allows you to resume a suspended participant in a specific room
    pub async fn resume(
        &self,
        room: u64,
        participant: u64,
        options: AudioBridgeResumeOptions,
        timeout: Duration,
    ) -> JaResult<()> {
        let response = self
            .handle
            .send_waiton_result::<AudioBridgePluginData>(
                serde_json::to_value(AudioBridgeResumeMsg::new(room, participant, options))?,
                timeout,
            )
            .await?;
        match response.event {
            AudioBridgePluginEvent::Success {} => Ok(()),
            _ => Err(JaError::UnexpectedResponse),
        }
    }

    /// Lists all the participants of a specific room and their details
    pub async fn list_participants(
        &self,
        room: u64,
        timeout: Duration,
    ) -> JaResult<(u64, Vec<Participant>)> {
        let response = self
            .handle
            .send_waiton_result::<AudioBridgePluginData>(
                serde_json::to_value(AudioBridgeListParticipantsMsg::new(room))?,
                timeout,
            )
            .await?;
        match response.event {
            AudioBridgePluginEvent::ListParticipants { room, participants } => {
                Ok((room, participants))
            }
            _ => Err(JaError::UnexpectedResponse),
        }
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
