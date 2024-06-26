use jarust::error::JaError;
use jarust::japrotocol::EstablishmentProtocol;
use jarust::japrotocol::GenericEvent;
use jarust::japrotocol::JaHandleEvent;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::ResponseType;
use serde::Deserialize;
use serde_json::from_value;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
enum EchoTestEventDto {
    #[serde(untagged)]
    Result { echotest: String, result: String },
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PluginEvent {
    EchoTestEvent(EchoTestEvent),
    GenericEvent(GenericEvent),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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

#[cfg(test)]
mod tests {
    use super::PluginEvent;
    use crate::echo_test::events::EchoTestEvent;
    use jarust::japrotocol::EstablishmentProtocol;
    use jarust::japrotocol::JaHandleEvent;
    use jarust::japrotocol::JaResponse;
    use jarust::japrotocol::Jsep;
    use jarust::japrotocol::JsepType;
    use jarust::japrotocol::PluginData;
    use jarust::japrotocol::ResponseType;
    use serde_json::json;

    #[test]
    fn it_parse_result_event() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.echotest".to_string(),
                    data: json!({
                        "echotest": "event",
                        "result": "ok"
                    }),
                },
            }),
            establishment_protocol: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::EchoTestEvent(EchoTestEvent::Result {
                echotest: "event".to_string(),
                result: "ok".to_string()
            })
        );
    }

    #[test]
    fn it_parse_result_with_establishment_event() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.echotest".to_string(),
                    data: json!({
                        "echotest": "event",
                        "result": "ok"
                    }),
                },
            }),
            establishment_protocol: Some(EstablishmentProtocol::JSEP(Jsep {
                jsep_type: JsepType::Answer,
                sdp: "test_sdp".to_string(),
            })),
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::EchoTestEvent(EchoTestEvent::ResultWithEstablishment {
                echotest: "event".to_string(),
                result: "ok".to_string(),
                establishment_protocol: EstablishmentProtocol::JSEP(Jsep {
                    jsep_type: JsepType::Answer,
                    sdp: "test_sdp".to_string()
                })
            })
        );
    }
}
