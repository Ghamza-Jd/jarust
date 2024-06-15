use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

/// The top-level response
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JaResponse {
    #[serde(flatten)]
    pub janus: ResponseType,
    pub transaction: Option<String>,
    pub session_id: Option<u64>,
    pub sender: Option<u64>,
    #[serde(flatten)]
    pub establishment_protocol: Option<EstablishmentProtocol>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "janus")]
pub enum ResponseType {
    #[serde(rename = "error")]
    Error { error: ErrorResponse },
    #[serde(rename = "server_info")]
    ServerInfo,
    #[serde(rename = "ack")]
    Ack,
    #[serde(rename = "success")]
    Success(JaSuccessProtocol),
    #[serde(untagged)]
    Event(JaHandleEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ErrorResponse {
    pub code: u16,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "janus")]
pub enum JaSuccessProtocol {
    #[serde(untagged)]
    Data { data: JaData },
    #[serde(untagged)]
    Plugin {
        #[serde(rename = "plugindata")]
        plugin_data: PluginData,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JaData {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PluginData {
    pub plugin: String,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "janus")]
pub enum JaHandleEvent {
    #[serde(rename = "event")]
    PluginEvent {
        #[serde(rename = "plugindata")]
        plugin_data: PluginData,
    },
    #[serde(untagged)]
    GenericEvent(GenericEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "janus")]
pub enum GenericEvent {
    #[serde(rename = "detached")]
    Detached,
    /// The PeerConnection was closed, either by Janus or by the user/application, and as such cannot be used anymore.
    #[serde(rename = "hangup")]
    Hangup,
    /// Whether Janus is receiving (receiving: true/false) audio/video (type: "audio/video") on this PeerConnection.
    #[serde(rename = "media")]
    Media,
    #[serde(rename = "timeout")]
    Timeout,
    /// ICE and DTLS succeeded, and so Janus correctly established a PeerConnection with the user/application.
    #[serde(rename = "webrtcup")]
    WebrtcUp,
    /// Whether Janus is reporting trouble sending/receiving (uplink: true/false) media on this PeerConnection.
    #[serde(rename = "slowlink")]
    Slowlink,
    #[serde(rename = "trickle")]
    Trickle,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum JsepType {
    #[serde(rename = "offer")]
    Offer,
    #[serde(rename = "answer")]
    Answer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Jsep {
    #[serde(rename = "type")]
    pub jsep_type: JsepType,
    pub sdp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RTP {
    pub ip: String,
    pub port: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiolevel_ext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fec: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EstablishmentProtocol {
    #[serde(rename = "jsep")]
    JSEP(Jsep),
    #[serde(rename = "rtp")]
    RTP(RTP),
}

#[cfg(test)]
mod tests {
    use super::JaData;
    use super::JaResponse;
    use super::JaSuccessProtocol;
    use super::ResponseType;
    use crate::japrotocol::EstablishmentProtocol;
    use crate::japrotocol::GenericEvent;
    use crate::japrotocol::JaHandleEvent;
    use crate::japrotocol::Jsep;
    use crate::japrotocol::JsepType;
    use crate::japrotocol::PluginData;
    use serde_json::json;

    #[test]
    fn it_parse_create_connection_rsp() {
        let rsp = json!({
            "janus": "success",
            "transaction": "7be89359-8c3f-44fc-93a6-72e35bb56058",
            "data": {
                "id": 5486640424129986u64
            }
        });
        let actual_rsp = serde_json::from_value::<JaResponse>(rsp).unwrap();
        let expected = JaResponse {
            janus: ResponseType::Success(JaSuccessProtocol::Data {
                data: JaData {
                    id: 5486640424129986u64,
                },
            }),
            transaction: Some("7be89359-8c3f-44fc-93a6-72e35bb56058".to_string()),
            sender: None,
            session_id: None,
            establishment_protocol: None,
        };
        assert_eq!(actual_rsp, expected);
    }

    #[test]
    fn it_parse_attach_rsp() {
        let rsp = json!({
            "janus": "success",
            "session_id": 1706796313061627u64,
            "transaction": "151f9362-3d12-45e5-ba02-b91a38be5a06",
            "data": {
                "id": 7548423276295183u64
            }
        });
        let actual_rsp = serde_json::from_value::<JaResponse>(rsp).unwrap();
        let expected = JaResponse {
            janus: ResponseType::Success(JaSuccessProtocol::Data {
                data: JaData {
                    id: 7548423276295183u64,
                },
            }),
            transaction: Some("151f9362-3d12-45e5-ba02-b91a38be5a06".to_string()),
            sender: None,
            session_id: Some(1706796313061627u64),
            establishment_protocol: None,
        };
        assert_eq!(actual_rsp, expected);
    }

    #[test]
    fn it_parse_echotest_event() {
        let event = json!({
            "janus": "event",
            "sender": 3010144072065778u64,
            "transaction": "c7bb120f-ed4e-4e00-b8de-bfc3e66f098e",
            "session_id": 8643988533991908u64,
            "plugindata": {
                "plugin": "janus.plugin.echotest",
                "data": {
                    "echotest": "event",
                    "result": "ok"
                }
            },
            "jsep": {
                "type": "answer",
                "sdp": "random_sdp"
            }
        });
        let actual_event = serde_json::from_value::<JaResponse>(event).unwrap();
        let expected = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.echotest".to_string(),
                    data: json!({
                        "echotest": "event",
                        "result": "ok"
                    }),
                },
            }),
            transaction: Some("c7bb120f-ed4e-4e00-b8de-bfc3e66f098e".to_string()),
            sender: Some(3010144072065778u64),
            session_id: Some(8643988533991908u64),
            establishment_protocol: Some(EstablishmentProtocol::JSEP(Jsep {
                sdp: "random_sdp".to_string(),
                jsep_type: JsepType::Answer,
            })),
        };
        assert_eq!(actual_event, expected);
    }

    #[test]
    fn it_parse_detached_event() {
        let event = json!({
            "janus": "detached",
            "sender": 5373520011480655u64,
            "session_id": 3889473834879521u64
        });
        let actual_event = serde_json::from_value::<JaResponse>(event).unwrap();
        let expected = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::GenericEvent(GenericEvent::Detached)),
            transaction: None,
            sender: Some(5373520011480655u64),
            session_id: Some(3889473834879521u64),
            establishment_protocol: None,
        };
        assert_eq!(actual_event, expected);
    }

    #[test]
    fn it_parse_webrtcup_event() {
        let event = json!({
            "janus": "webrtcup",
            "sender": 2676358135723942u64,
            "session_id": 1942958911060866u64
        });
        let actual_event = serde_json::from_value::<JaResponse>(event).unwrap();
        let expected = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::GenericEvent(GenericEvent::WebrtcUp)),
            transaction: None,
            sender: Some(2676358135723942u64),
            session_id: Some(1942958911060866u64),
            establishment_protocol: None,
        };
        assert_eq!(actual_event, expected);
    }
}
