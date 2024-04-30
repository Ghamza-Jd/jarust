use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AudioBridgePluginData {
    pub plugin: String,
    #[serde(rename = "data")]
    pub event: AudioBridgePluginEvent,
}

#[derive(Debug, Deserialize)]
pub struct RoomCreated {
    pub room: u64,
    pub permanent: bool,
}

#[derive(Debug, Deserialize)]
pub struct RoomEdited {
    pub room: u64,
    pub permanent: bool,
}

#[derive(Debug, Deserialize)]
pub struct RoomDestroyed {
    pub room: u64,
    pub permanent: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListRooms {
    pub list: Vec<Room>,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    pub room: u64,
    pub description: String,
    pub pin_required: bool,
    pub sampling_rate: u64,
    pub spatial_audio: Option<bool>,
    pub record: bool,
    pub num_participants: u64,
    pub muted: bool,
}

#[derive(Debug, Deserialize)]
pub struct Participant {
    pub id: String,
    pub display: String,
    pub setup: bool,
    pub muted: bool,
    pub suspended: bool,
    pub talking: bool,
    pub spatial_position: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "audiobridge")]
pub enum AudioBridgePluginEvent {
    #[serde(rename = "participants")]
    ListParticipants {
        room: u64,
        participants: Vec<Participant>,
    },
    #[serde(rename = "success")]
    #[serde(untagged)]
    List { list: Vec<Room> },
    #[serde(rename = "success")]
    #[serde(untagged)]
    Allowed { room: u64, allowed: Vec<String> },
    #[serde(rename = "success")]
    #[serde(untagged)]
    ExistsRoom { room: u64, exists: bool },
    #[serde(rename = "success")]
    #[serde(untagged)]
    Success {},
}
