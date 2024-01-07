use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AudioBridgePluginData {
    pub plugin: String,
    #[serde(rename = "data")]
    pub event: AudioBridgePluginEvent,
}

#[derive(Debug, Deserialize)]
pub enum AudioBridgePluginEvent {
    #[serde(untagged)]
    List {
        audiobridge: String,
        rooms: Vec<Room>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Room {
    room: String,
    description: String,
    pin_required: bool,
    sampling_rate: u64,
    spatial_audio: bool,
    record: bool,
    num_participants: u64,
}
