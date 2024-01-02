use serde::Serialize;

#[derive(Serialize)]
pub struct EchoTestStartMsg {
    pub audio: bool,
    pub video: bool,
}
