use crate::japrotocol::JaIdk;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_json::Value;

pub fn generate_transaction() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}

pub fn get_subnamespace_from_request(request: &Value) -> Option<String> {
    if let (Some(session_id), Some(handle_id)) = (
        request["session_id"].as_u64(),
        request["handle_id"].as_u64(),
    ) {
        Some(format!("{session_id}/{handle_id}"))
    } else {
        request["session_id"]
            .as_u64()
            .map(|session_id| format!("{session_id}"))
    }
}

pub fn get_subnamespace_from_response(response: JaIdk) -> Option<String> {
    if let Some(session_id) = response.session_id {
        if let Some(sender) = response.sender {
            Some(format!("{session_id}/{sender}"))
        } else {
            Some(format!("{session_id}"))
        }
    } else {
        None
    }
}
