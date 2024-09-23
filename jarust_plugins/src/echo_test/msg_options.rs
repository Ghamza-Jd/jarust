use serde::Serialize;

impl_tryfrom_serde_value!(EchoTestStartOptions);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize)]
pub struct EchoTestStartOptions {
    pub audio: bool,
    pub video: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
}
