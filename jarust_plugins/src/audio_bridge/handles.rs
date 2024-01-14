use super::{
    messages::{
        AudioBridgeAction, AudioBridgeAllowedMsg, AudioBridgeAllowedOptions, AudioBridgeCreateMsg,
        AudioBridgeCreateOptions, AudioBridgeDestroyMsg, AudioBridgeDestroyOptions,
        AudioBridgeEditMsg, AudioBridgeEditOptions, AudioBridgeExistsMsg, AudioBridgeJoinMsg,
        AudioBridgeJoinOptions, AudioBridgeKickAllMsg, AudioBridgeKickAllOptions,
        AudioBridgeKickMsg, AudioBridgeKickOptions, AudioBridgeListMsg,
        AudioBridgeListParticipantsMsg, AudioBridgeResumeMsg, AudioBridgeResumeOptions,
        AudioBridgeSuspendMsg, AudioBridgeSuspendOptions,
    },
    results::{AudioBridgePluginData, AudioBridgePluginEvent, Participant, Room},
};
use jarust::{japrotocol::EstablishmentProtocol, prelude::*};
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
                AudioBridgeCreateMsg::new(AudioBridgeCreateOptions {
                    room,
                    ..Default::default()
                }),
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
        options: AudioBridgeCreateOptions,
    ) -> JaResult<(u64, bool)> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeCreateMsg::new(options),
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

    pub async fn edit_room(&self, room: u64, options: AudioBridgeEditOptions) -> JaResult<u64> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeEditMsg::new(room, options),
            )?)
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::EditRoom { room, .. } => room,
            _ => {
                panic!("Unexpected Response!")
            }
        };

        Ok(result)
    }

    pub async fn destroy_room(
        &self,
        room: u64,
        options: AudioBridgeDestroyOptions,
    ) -> JaResult<(u64, bool)> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeDestroyMsg::new(room, options),
            )?)
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::DestroyRoom {
                room, permanent, ..
            } => (room, permanent),
            _ => {
                panic!("Unexpected Response!")
            }
        };

        Ok(result)
    }

    pub async fn join_room(
        &self,
        room: u64,
        options: AudioBridgeJoinOptions,
        protocol: Option<EstablishmentProtocol>,
    ) -> JaResult<()> {
        match protocol {
            Some(protocol) => {
                self.handle
                    .message_with_establishment_protocol(
                        serde_json::to_value(AudioBridgeJoinMsg::new(room, options))?,
                        protocol,
                    )
                    .await
            }
            None => {
                self.handle
                    .message_with_ack(serde_json::to_value(AudioBridgeJoinMsg::new(
                        room, options,
                    ))?)
                    .await
            }
        }
        .map(|_| ())
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

    pub async fn allowed(
        &self,
        room: u64,
        action: AudioBridgeAction,
        allowed: Vec<String>,
        options: AudioBridgeAllowedOptions,
    ) -> JaResult<(u64, Vec<String>)> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeAllowedMsg::new(room, action, allowed, options),
            )?)
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::Allowed { room, allowed, .. } => (room, allowed),
            _ => {
                panic!("Unexpected Response!")
            }
        };
        Ok(result)
    }

    pub async fn exists(&self, room: u64) -> JaResult<bool> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeExistsMsg::new(room),
            )?)
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::ExistsRoom { exists, .. } => exists,
            _ => {
                panic!("Unexpected Response!")
            }
        };
        Ok(result)
    }

    pub async fn kick(
        &self,
        room: u64,
        participant: u64,
        options: AudioBridgeKickOptions,
    ) -> JaResult<()> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeKickMsg::new(room, participant, options),
            )?)
            .await?;
        match response.event {
            AudioBridgePluginEvent::Success {} => Ok(()),
            _ => {
                panic!("Unexpected Response!")
            }
        }
    }

    pub async fn kick_all(&self, room: u64, options: AudioBridgeKickAllOptions) -> JaResult<()> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeKickAllMsg::new(room, options),
            )?)
            .await?;
        match response.event {
            AudioBridgePluginEvent::Success {} => Ok(()),
            _ => {
                panic!("Unexpected Response!")
            }
        }
    }

    pub async fn suspend(
        &self,
        room: u64,
        participant: u64,
        options: AudioBridgeSuspendOptions,
    ) -> JaResult<()> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeSuspendMsg::new(room, participant, options),
            )?)
            .await?;
        match response.event {
            AudioBridgePluginEvent::Success {} => Ok(()),
            _ => {
                panic!("Unexpected Response!")
            }
        }
    }

    pub async fn resume(
        &self,
        room: u64,
        participant: u64,
        options: AudioBridgeResumeOptions,
    ) -> JaResult<()> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeResumeMsg::new(room, participant, options),
            )?)
            .await?;
        match response.event {
            AudioBridgePluginEvent::Success {} => Ok(()),
            _ => {
                panic!("Unexpected Response!")
            }
        }
    }

    pub async fn list_participants(&self, room: u64) -> JaResult<(u64, Vec<Participant>)> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeListParticipantsMsg::new(room),
            )?)
            .await?;
        match response.event {
            AudioBridgePluginEvent::ListParticipants { room, participants } => {
                Ok((room, participants))
            }
            _ => {
                panic!("Unexpected Response!")
            }
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
