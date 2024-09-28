use super::common::AudioBridgeParticipant;
use crate::JanusId;
use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(untagged)]
pub enum AudioBridgeRespone<T> {
    Error { error_code: u16, error: String },
    Response(T),
}

impl<T> AudioBridgeRespone<T> {
    pub fn map_err(self) -> Result<T, super::Error> {
        match self {
            AudioBridgeRespone::Response(t) => Ok(t),
            AudioBridgeRespone::Error { error, error_code } => {
                Err(super::Error::AudioBridge { error_code, error })
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct AudioBridgeRoomCreatedRsp {
    pub room: JanusId,
    pub permanent: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct AudioBridgeRoomEditedRsp {
    pub room: JanusId,
    pub permanent: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct AudioBridgeRoomDestroyedRsp {
    pub room: JanusId,
    pub permanent: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct AudioBridgeListRoomsRsp {
    pub list: Vec<AudioBridgeRoom>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct AudioBridgeRoom {
    pub room: JanusId,
    pub description: String,
    pub pin_required: bool,
    pub sampling_rate: u64,
    pub spatial_audio: Option<bool>,
    pub record: bool,
    pub num_participants: u64,
    pub muted: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct AudioBridgeAllowedRsp {
    pub room: JanusId,
    pub allowed: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct AudioBridgeExistsRoomRsp {
    pub room: JanusId,
    pub exists: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct AudioBridgeListParticipantsRsp {
    pub room: JanusId,
    pub participants: Vec<AudioBridgeParticipant>,
}
