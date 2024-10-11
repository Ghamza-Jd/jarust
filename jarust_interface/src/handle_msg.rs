use super::japrotocol::EstProto;
use serde_json::Value;

pub struct HandleMessage {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
}

pub struct HandleMessageWithEst {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
    pub estproto: EstProto,
}
