use jarust_interface::japrotocol::GenericEvent;
use jarust_interface::japrotocol::JaHandleEvent;
use jarust_interface::japrotocol::JaResponse;
use jarust_interface::japrotocol::Jsep;
use jarust_interface::japrotocol::PluginInnerData;
use jarust_interface::japrotocol::ResponseType;
use serde::Deserialize;
use serde_json::from_value;
use serde_json::Value;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
enum EchoTestEventDto {
    #[serde(untagged)]
    Result { echotest: String, result: String },
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum PluginEvent {
    EchoTestEvent(EchoTestEvent),
    GenericEvent(GenericEvent),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EchoTestEvent {
    Result {
        echotest: String,
        result: String,
    },
    ResultWithEst {
        echotest: String,
        result: String,
        jsep: Jsep,
    },
    Error {
        error_code: u16,
        error: String,
    },
    Other(Value),
}

impl TryFrom<JaResponse> for PluginEvent {
    type Error = jarust_interface::Error;

    fn try_from(value: JaResponse) -> Result<Self, Self::Error> {
        match value.janus {
            ResponseType::Event(JaHandleEvent::PluginEvent { plugin_data }) => {
                let echotest_event = match plugin_data.data {
                    PluginInnerData::Error { error_code, error } => {
                        EchoTestEvent::Error { error_code, error }
                    }
                    PluginInnerData::Data(data) => {
                        match from_value::<EchoTestEventDto>(data.clone()) {
                            Ok(EchoTestEventDto::Result { echotest, result }) => match value.jsep {
                                Some(jsep) => EchoTestEvent::ResultWithEst {
                                    echotest,
                                    result,
                                    jsep,
                                },
                                None => EchoTestEvent::Result { echotest, result },
                            },
                            Err(_) => EchoTestEvent::Other(data),
                        }
                    }
                };
                Ok(PluginEvent::EchoTestEvent(echotest_event))
            }
            ResponseType::Event(JaHandleEvent::GenericEvent(event)) => {
                Ok(PluginEvent::GenericEvent(event))
            }
            _ => Err(Self::Error::IncompletePacket),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PluginEvent;
    use crate::echo_test::events::EchoTestEvent;
    use jarust_interface::japrotocol::JaHandleEvent;
    use jarust_interface::japrotocol::JaResponse;
    use jarust_interface::japrotocol::Jsep;
    use jarust_interface::japrotocol::JsepType;
    use jarust_interface::japrotocol::PluginData;
    use jarust_interface::japrotocol::PluginInnerData;
    use jarust_interface::japrotocol::ResponseType;
    use serde_json::json;

    #[test]
    fn it_parse_result_event() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.echotest".to_string(),
                    data: PluginInnerData::Data(json!({
                        "echotest": "event",
                        "result": "ok"
                    })),
                },
            }),
            jsep: None,
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
    fn it_parse_result_with_jsep_event() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.echotest".to_string(),
                    data: PluginInnerData::Data(json!({
                        "echotest": "event",
                        "result": "ok"
                    })),
                },
            }),
            jsep: Some(Jsep {
                jsep_type: JsepType::Answer,
                trickle: Some(false),
                sdp: "test_sdp".to_string(),
            }),
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::EchoTestEvent(EchoTestEvent::ResultWithEst {
                echotest: "event".to_string(),
                result: "ok".to_string(),
                jsep: Jsep {
                    jsep_type: JsepType::Answer,
                    trickle: Some(false),
                    sdp: "test_sdp".to_string()
                }
            })
        );
    }

    #[test]
    fn it_parse_error_event() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.echotest".to_string(),
                    data: PluginInnerData::Error {
                        error_code: 404,
                        error: "Plugin not found".to_owned(),
                    },
                },
            }),
            jsep: Some(Jsep {
                jsep_type: JsepType::Answer,
                trickle: Some(false),
                sdp: "test_sdp".to_string(),
            }),
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::EchoTestEvent(EchoTestEvent::Error {
                error_code: 404,
                error: "Plugin not found".to_owned()
            })
        );
    }

    #[test]
    fn it_parse_unsupported_event_as_other() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.echotest".to_string(),
                    data: PluginInnerData::Data(json!({
                        "echotest": "jarust_rocks",
                        "jarust": "rocks"
                    })),
                },
            }),
            jsep: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::EchoTestEvent(EchoTestEvent::Other(json!({
                "echotest": "jarust_rocks",
                "jarust": "rocks"
            })))
        );
    }
}
