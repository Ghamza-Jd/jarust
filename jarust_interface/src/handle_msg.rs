use super::japrotocol::EstProto;
use serde_json::Value;
use std::time::Duration;

pub struct HandleMessage {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
}

pub struct HandleMessageWithTimeout {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
    pub timeout: Duration,
}

pub struct HandleMessageWithEst {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
    pub estproto: EstProto,
}

pub struct HandleMessageWithEstAndTimeout {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
    pub timeout: Duration,
    pub estproto: EstProto,
}
