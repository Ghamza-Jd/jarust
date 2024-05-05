use jarust::error::JaError;
use jarust::japrotocol::GenericEvent;
use jarust::japrotocol::JaHandleEvent;
use serde::Deserialize;

pub enum Events {
    PluginEvent(PluginEvent),
    GenericEvent(GenericEvent),
}

#[derive(Debug, Deserialize)]
pub enum PluginEvent {
    #[serde(untagged)]
    Result { echotest: String, result: String },
}

impl TryFrom<JaHandleEvent> for Events {
    type Error = JaError;

    fn try_from(value: JaHandleEvent) -> Result<Self, Self::Error> {
        let event = match value {
            JaHandleEvent::PluginEvent { plugin_data } => {
                Self::PluginEvent(serde_json::from_value::<PluginEvent>(plugin_data.data)?)
            }
            JaHandleEvent::GenericEvent(event) => Self::GenericEvent(event),
        };
        Ok(event)
    }
}
