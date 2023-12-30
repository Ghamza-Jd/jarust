use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EchoTestPluginData {
    pub plugin: String,
    #[serde(rename = "data")]
    pub event: EchoTestPluginEvent,
}

#[derive(Debug, Deserialize)]
pub enum EchoTestPluginEvent {
    #[serde(untagged)]
    Result { echotest: String, result: String },
}
