use serde::Deserialize;

use crate::JanusId;

use super::common::Participant;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RoomCreatedRsp {
    pub room: JanusId,
    pub permanent: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RoomEditedRsp {
    pub room: JanusId,
    pub permanent: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RoomDestroyedRsp {
    pub room: JanusId,
    pub permanent: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct ListRoomsRsp {
    pub list: Vec<Room>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct Room {
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
pub struct AllowedRsp {
    pub room: JanusId,
    pub allowed: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct ExistsRoomRsp {
    pub room: JanusId,
    pub exists: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct ListParticipantsRsp {
    pub room: JanusId,
    pub participants: Vec<Participant>,
}
