use serde::Deserialize;

use crate::Identifier;

use super::common::Participant;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RoomCreatedRsp {
    pub room: Identifier,
    pub permanent: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RoomEditedRsp {
    pub room: Identifier,
    pub permanent: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RoomDestroyedRsp {
    pub room: Identifier,
    pub permanent: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct ListRoomsRsp {
    pub list: Vec<Room>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct Room {
    pub room: Identifier,
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
    pub room: Identifier,
    pub allowed: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct ExistsRoomRsp {
    pub room: Identifier,
    pub exists: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct ListParticipantsRsp {
    pub room: Identifier,
    pub participants: Vec<Participant>,
}
