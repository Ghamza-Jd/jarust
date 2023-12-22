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
    } else if let Some(session_id) = request["session_id"].as_u64() {
        Some(format!("{session_id}"))
    } else {
        None
    }
}

pub fn get_subnamespace_from_response(response: &Value) -> Option<String> {
    if let (Some(session_id), Some(sender)) =
        (response["session_id"].as_u64(), response["sender"].as_u64())
    {
        Some(format!("{session_id}/{sender}"))
    } else if let Some(session_id) = response["session_id"].as_u64() {
        Some(format!("{session_id}"))
    } else {
        None
    }
}
