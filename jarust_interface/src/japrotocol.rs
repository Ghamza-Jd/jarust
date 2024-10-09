use super::respones::ServerInfoRsp;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

/// The top-level response
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct JaResponse {
    #[serde(flatten)]
    pub janus: ResponseType,
    pub transaction: Option<String>,
    pub session_id: Option<u64>,
    pub sender: Option<u64>,
    #[serde(flatten)]
    pub estproto: Option<EstProto>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(tag = "janus")]
pub enum ResponseType {
    #[serde(rename = "error")]
    Error { error: ErrorResponse },
    // Wrapped in a box so the entire enum isn't as large as this variant
    // e.g: Ack that doesn't contain any data will have allocate the same space as ServerInfo
    #[serde(rename = "server_info")]
    ServerInfo(Box<ServerInfoRsp>),
    #[serde(rename = "ack")]
    Ack,
    #[serde(rename = "success")]
    Success(JaSuccessProtocol),
    #[serde(untagged)]
    Event(JaHandleEvent),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub reason: String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
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

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PluginData {
    pub plugin: String,
    pub data: PluginInnerData,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginInnerData {
    Error { error_code: u16, error: String },
    // Gotcha
    // The pattern matching is done in the order of the variants,
    // this field wraps a generic value, so it will always match.
    Data(Value),
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum JsepType {
    #[serde(rename = "offer")]
    Offer,
    #[serde(rename = "answer")]
    Answer,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct Jsep {
    #[serde(rename = "type")]
    pub jsep_type: JsepType,
    pub trickle: Option<bool>,
    pub sdp: String,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
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

/// Establishment Protocol
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum EstProto {
    #[serde(rename = "jsep")]
    JSEP(Jsep),
    #[serde(rename = "rtp")]
    RTP(RTP),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct Candidate {
    #[serde(rename = "sdpMid")]
    pub sdp_mid: String,
    #[serde(rename = "sdpMLineIndex")]
    pub sdp_mline_index: String,
    pub candidate: String,
}

#[cfg(test)]
mod tests {
    use super::EstProto;
    use super::GenericEvent;
    use super::JaData;
    use super::JaHandleEvent;
    use super::JaResponse;
    use super::JaSuccessProtocol;
    use super::Jsep;
    use super::JsepType;
    use super::PluginData;
    use super::ResponseType;
    use crate::japrotocol::PluginInnerData;
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
            estproto: None,
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
            estproto: None,
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
                    data: PluginInnerData::Data(json!({
                        "echotest": "event",
                        "result": "ok"
                    })),
                },
            }),
            transaction: Some("c7bb120f-ed4e-4e00-b8de-bfc3e66f098e".to_string()),
            sender: Some(3010144072065778u64),
            session_id: Some(8643988533991908u64),
            estproto: Some(EstProto::JSEP(Jsep {
                sdp: "random_sdp".to_string(),
                trickle: None,
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
            estproto: None,
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
            estproto: None,
        };
        assert_eq!(actual_event, expected);
    }

    #[test]
    fn it_parse_error_event() {
        let event = json!({
           "janus": "success",
           "session_id": 2158724686674557u64,
           "transaction": "nNbmsbj33zLY",
           "sender": 77797716144085u64,
           "plugindata": {
              "plugin": "janus.plugin.streaming",
              "data": {
                 "error_code": 456,
                 "error": "Can't add 'rtp' stream, error creating data source stream"
              }
           }
        });
        let actual_event = serde_json::from_value::<JaResponse>(event).unwrap();
        let expected = JaResponse {
            janus: ResponseType::Success(JaSuccessProtocol::Plugin {
                plugin_data: PluginData {
                    plugin: "janus.plugin.streaming".to_string(),
                    data: PluginInnerData::Error {
                        error_code: 456,
                        error: "Can't add 'rtp' stream, error creating data source stream"
                            .to_string(),
                    },
                },
            }),
            transaction: Some("nNbmsbj33zLY".to_string()),
            sender: Some(77797716144085u64),
            session_id: Some(2158724686674557u64),
            estproto: None,
        };
        assert_eq!(actual_event, expected);
    }
}
