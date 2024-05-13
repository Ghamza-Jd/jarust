use super::mock_transport::MockServer;
use jarust::japrotocol::JaData;
use jarust::japrotocol::JaSuccessProtocol;
use jarust::japrotocol::ResponseType;
use jarust::prelude::*;
use tokio::sync::mpsc;

pub struct MockHandleConfig {
    pub session_id: u64,
    pub handle_id: u64,
    pub plugin_id: String,
}

#[allow(dead_code)]
pub async fn mock_handle(
    session: JaSession,
    server: &MockServer,
    config: MockHandleConfig,
) -> JaResult<(JaHandle, mpsc::UnboundedReceiver<JaResponse>)> {
    let attachment_msg = serde_json::to_string(&JaResponse {
        janus: ResponseType::Success(JaSuccessProtocol::Data {
            data: JaData {
                id: config.handle_id,
            },
        }),
        transaction: None,
        session_id: Some(config.session_id),
        sender: None,
        establishment_protocol: None,
    })
    .unwrap();
    server.mock_send_to_client(&attachment_msg).await;
    let (handle, stream) = session.attach(&config.plugin_id).await?;

    Ok((handle, stream))
}
