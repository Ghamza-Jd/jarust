use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum EchoTestPluginEvent {
    #[serde(untagged)]
    Result { echotest: String, result: String },
}
