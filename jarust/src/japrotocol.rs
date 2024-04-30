use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub enum JaConnectionRequestProtocol {
    #[serde(rename = "info")]
    ServerInfo,
    #[serde(rename = "create")]
    CreateSession,
}

#[derive(Serialize)]
pub enum JaSessionRequestProtocol {
    #[serde(rename = "keepalive")]
    KeepAlive,
    #[serde(rename = "destory")]
    DestorySession,
    #[serde(rename = "claim")]
    Claim,
    #[serde(rename = "attach")]
    AttachPlugin,
}

#[derive(Serialize)]
pub enum JaHandleRequestProtocol {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "trickle")]
    Trickle,
    #[serde(rename = "hangup")]
    Hangup,
    #[serde(rename = "detach")]
    DetachPlugin,
}

/// The top-level response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JaResponse {
    #[serde(flatten)]
    pub janus: JaResponseProtocol,
    pub transaction: Option<String>,
    pub session_id: Option<u64>,
    pub sender: Option<u64>,
    #[serde(flatten)]
    pub establishment_protocol: Option<EstablishmentProtocol>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "janus")]
pub enum JaResponseProtocol {
    #[serde(rename = "error")]
    Error { error: JaResponseError },
    #[serde(rename = "server_info")]
    ServerInfo,
    #[serde(rename = "ack")]
    Ack,
    #[serde(rename = "success")]
    Success(JaSuccessProtocol),
    #[serde(untagged)]
    Event(JaEventProtocol),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JaData {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JaResponseError {
    pub code: u16,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginData {
    pub plugin: String,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "janus")]
pub enum JaEventProtocol {
    #[serde(rename = "event")]
    Event {
        #[serde(rename = "plugindata")]
        plugin_data: PluginData,
    },
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Jsep {
    #[serde(rename = "type")]
    pub jsep_type: JsepType,
    pub sdp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EstablishmentProtocol {
    #[serde(rename = "jsep")]
    JSEP(Jsep),
    #[serde(rename = "rtp")]
    RTP(RTP),
}
