use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AudioBridgePluginData {
    pub plugin: String,
    #[serde(rename = "data")]
    pub event: AudioBridgePluginEvent,
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
pub enum AudioBridgePluginEvent {
    #[serde(untagged)]
    List {
        audiobridge: String,
        list: Vec<Room>,
    },
    #[serde(untagged)]
    CreateRoom {
        audiobridge: String,
        room: u64,
        permanent: bool,
    },
}
