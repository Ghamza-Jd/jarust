use serde::Serialize;

tryfrom_serde_value!(StartOptions);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize)]
pub struct StartOptions {
    pub audio: bool,
    pub video: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
}
