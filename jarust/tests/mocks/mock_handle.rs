use super::mock_transport::MockServer;
use jarust::params::AttachHandleParams;
use jarust::prelude::*;
use jarust_transport_next::japrotocol::JaData;
use jarust_transport_next::japrotocol::JaSuccessProtocol;
use jarust_transport_next::japrotocol::ResponseType;
use std::time::Duration;
use tokio::sync::mpsc;

pub struct MockHandleConfig {
    pub session_id: u64,
    pub handle_id: u64,
    pub plugin_id: String,
    pub timeout: Duration,
}

#[allow(dead_code)]
pub async fn mock_handle(
    session: JaSession,
    server: &MockServer,
    config: MockHandleConfig,
    expected_transaction: &str,
) -> JaResult<(JaHandle, mpsc::UnboundedReceiver<JaResponse>)> {
    let attachment_msg = serde_json::to_string(&JaResponse {
        janus: ResponseType::Success(JaSuccessProtocol::Data {
            data: JaData {
                id: config.handle_id,
            },
        }),
        transaction: Some(expected_transaction.to_string()),
        session_id: Some(config.session_id),
        sender: None,
        establishment_protocol: None,
    })
    .unwrap();
    server.mock_send_to_client(&attachment_msg).await;
    let (handle, stream) = session
        .attach(AttachHandleParams {
            plugin_id: config.plugin_id,
            timeout: config.timeout,
        })
        .await?;

    Ok((handle, stream))
}
