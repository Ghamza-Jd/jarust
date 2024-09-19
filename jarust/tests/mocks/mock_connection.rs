use super::mock_interface::MockInterface;
use jarust::jaconnection::JaConnection;
use jarust::prelude::JaResult;

#[allow(dead_code)]
pub async fn mock_connection(interface: MockInterface) -> JaResult<JaConnection> {
    let connection = jarust::custom_connect(interface).await?;
    Ok(connection)
}
