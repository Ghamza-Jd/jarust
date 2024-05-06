use jarust::error::JaError;
use jarust::japrotocol::EstablishmentProtocol;
use jarust::japrotocol::GenericEvent;
use jarust::japrotocol::JaHandleEvent;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::ResponseType;
use serde::Deserialize;
use serde_json::from_value;

#[derive(Debug, Deserialize, Clone)]
enum EchoTestEventDto {
    #[serde(untagged)]
    Result { echotest: String, result: String },
}

pub enum PluginEvent {
    EchoTestEvent(EchoTestEvent),
    GenericEvent(GenericEvent),
}

pub enum EchoTestEvent {
    Result {
        echotest: String,
        result: String,
    },
    ResultWithEstablishment {
        echotest: String,
        result: String,
        establishment_protocol: EstablishmentProtocol,
    },
}

impl TryFrom<JaResponse> for PluginEvent {
    type Error = JaError;

    fn try_from(value: JaResponse) -> Result<Self, Self::Error> {
        match value.janus {
            ResponseType::Event(JaHandleEvent::PluginEvent { plugin_data }) => {
                let echotest_event = match from_value::<EchoTestEventDto>(plugin_data.data)? {
                    EchoTestEventDto::Result { echotest, result } => {
                        match value.establishment_protocol {
                            Some(establishment_protocol) => {
                                EchoTestEvent::ResultWithEstablishment {
                                    echotest,
                                    result,
                                    establishment_protocol,
                                }
                            }
                            None => EchoTestEvent::Result { echotest, result },
                        }
                    }
                };
                Ok(PluginEvent::EchoTestEvent(echotest_event))
            }
            ResponseType::Event(JaHandleEvent::GenericEvent(event)) => {
                Ok(PluginEvent::GenericEvent(event))
            }
            _ => Err(JaError::IncompletePacket),
        }
    }
}
