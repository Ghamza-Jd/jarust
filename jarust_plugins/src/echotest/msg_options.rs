use serde::Serialize;

#[derive(Serialize, Default)]
pub struct StartOptions {
    pub audio: bool,
    pub video: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
}