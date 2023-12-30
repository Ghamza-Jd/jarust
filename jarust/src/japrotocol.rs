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
#[derive(Debug, Deserialize, Clone)]
pub struct JaResponse {
    #[serde(flatten)]
    pub janus: JaResponseProtocol,
    pub transaction: Option<String>,
    pub session_id: Option<u64>,
    pub sender: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "janus")]
pub enum JaResponseProtocol {
    #[serde(rename = "success")]
    Success { data: JaData },
    #[serde(rename = "error")]
    Error { error: JaResponseError },
    #[serde(rename = "server_info")]
    ServerInfo,
    #[serde(rename = "ack")]
    Ack,
    #[serde(untagged)]
    Event(JaEventProtocol),
}

#[derive(Debug, Deserialize, Clone)]
pub struct JaData {
    pub id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JaResponseError {
    pub code: u16,
    pub reason: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "janus")]
pub enum JaEventProtocol {
    #[serde(rename = "event")]
    Event {
        #[serde(rename = "plugindata")]
        plugin_data: Value,
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

#[derive(Serialize, Deserialize, Debug)]
pub enum JsepType {
    #[serde(rename = "offer")]
    Offer,
    #[serde(rename = "answer")]
    Answer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Jsep {
    #[serde(rename = "type")]
    pub jsep_type: JsepType,
    pub sdp: String,
}
