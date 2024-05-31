use jarust::error::JaError;
use jarust::japrotocol::GenericEvent;
use jarust::japrotocol::JaResponse;

#[derive(Debug, PartialEq)]
pub enum PluginEvent {
    GenericEvent(GenericEvent),
}

impl TryFrom<JaResponse> for PluginEvent {
    type Error = JaError;

    fn try_from(_value: JaResponse) -> Result<Self, Self::Error> {
        todo!()
    }
}
