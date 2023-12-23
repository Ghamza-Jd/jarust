use serde::Deserialize;

#[derive(Deserialize)]
pub struct AttachResponse {
    pub data: AttachInnerResponse,
}

#[derive(Deserialize)]
pub struct AttachInnerResponse {
    pub id: u64,
}

#[derive(Deserialize)]
pub struct CreateSessionResponse {
    pub data: CreateSessionInnerResponse,
}

#[derive(Deserialize)]
pub struct CreateSessionInnerResponse {
    pub id: u64,
}
