use super::mock_session::mock_session;
use super::mock_transport::MockServer;
use jarust::japrotocol::JaData;
use jarust::japrotocol::JaSuccessProtocol;
use jarust::japrotocol::ResponseType;
use jarust::prelude::*;
use tokio::sync::mpsc;

#[allow(dead_code)]
pub async fn mock_handle() -> JaResult<(JaHandle, mpsc::UnboundedReceiver<JaResponse>, MockServer)>
{
    let (session, server) = mock_session().await?;

    let attachment_msg = serde_json::to_string(&JaResponse {
        janus: ResponseType::Success(JaSuccessProtocol::Data {
            data: JaData { id: 3 },
        }),
        transaction: None,
        session_id: Some(2),
        sender: None,
        establishment_protocol: None,
    })
    .unwrap();
    server.mock_send_to_client(&attachment_msg).await;
    let (handle, stream) = session.attach("mock.plugin.test").await?;

    Ok((handle, stream, server))
}
