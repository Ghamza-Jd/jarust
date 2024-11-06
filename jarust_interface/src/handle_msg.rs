use crate::japrotocol::Jsep;
use serde_json::Value;

pub struct HandleMessage {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
}

pub struct HandleMessageWithJsep {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
    pub jsep: Jsep,
}
