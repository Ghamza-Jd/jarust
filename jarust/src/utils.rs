use crate::japrotocol::JaResponse;
use serde_json::Value;

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

pub fn get_subnamespace_from_response(response: JaResponse) -> Option<String> {
    let Some(session_id) = response.session_id else {
        return None;
    };
    let Some(sender) = response.sender else {
        return Some(format!("{session_id}"));
    };
    Some(format!("{session_id}/{sender}"))
}
