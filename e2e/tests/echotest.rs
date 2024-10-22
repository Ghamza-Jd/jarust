use jarust::jaconfig::JaConfig;
use jarust::jaconfig::JanusAPI;
use jarust_interface::tgenerator::RandomTransactionGenerator;
use jarust_plugins::echo_test::events::EchoTestEvent;
use jarust_plugins::echo_test::events::PluginEvent;
use jarust_plugins::echo_test::jahandle_ext::EchoTest;
use jarust_plugins::echo_test::params::EchoTestStartParams;
use std::time::Duration;

#[tokio::test]
async fn echotest_ws_e2e() {
    let config = JaConfig {
        url: "ws://localhost:8188/ws".to_string(),
        apisecret: None,
        server_root: "janus".to_string(),
        capacity: 32,
    };
    let mut connection = jarust::connect(config, JanusAPI::WebSocket, RandomTransactionGenerator)
        .await
        .expect("Failed to connect to server");
    let timeout = Duration::from_secs(10);
    let session = connection
        .create_session(10, Duration::from_secs(10))
        .await
        .expect("Failed to create session");
    let (handle, mut event_receiver) = session
        .attach_echo_test(timeout)
        .await
        .expect("Failed to attach plugin");

    handle
        .start(EchoTestStartParams {
            audio: Some(true),
            ..Default::default()
        })
        .await
        .expect("Failed to send start message");
    assert_eq!(
        event_receiver.recv().await,
        Some(PluginEvent::EchoTestEvent(EchoTestEvent::Result {
            echotest: "event".to_string(),
            result: "ok".to_string()
        }))
    );

    // Empty body should return an error
    handle
        .start(Default::default())
        .await
        .expect("Failed to send start message");
    assert!(matches!(
        event_receiver.recv().await,
        Some(PluginEvent::EchoTestEvent(EchoTestEvent::Error {
            error_code: _,
            error: _
        }))
    ));
}

#[tokio::test]
async fn echotest_rest_e2e() {
    let config = JaConfig {
        url: "http://localhost:8088".to_string(),
        apisecret: None,
        server_root: "janus".to_string(),
        capacity: 32,
    };
    let mut connection = jarust::connect(config, JanusAPI::Restful, RandomTransactionGenerator)
        .await
        .expect("Failed to connect to server");
    let timeout = Duration::from_secs(10);
    let session = connection
        .create_session(10, Duration::from_secs(10))
        .await
        .expect("Failed to create session");
    let (handle, mut event_receiver) = session
        .attach_echo_test(timeout)
        .await
        .expect("Failed to attach plugin");

    handle
        .start(EchoTestStartParams {
            audio: Some(true),
            ..Default::default()
        })
        .await
        .expect("Failed to send start message");
    assert_eq!(
        event_receiver.recv().await,
        Some(PluginEvent::EchoTestEvent(EchoTestEvent::Result {
            echotest: "event".to_string(),
            result: "ok".to_string()
        }))
    );

    // Empty body should return an error
    handle
        .start(Default::default())
        .await
        .expect("Failed to send start message");
    assert!(matches!(
        event_receiver.recv().await,
        Some(PluginEvent::EchoTestEvent(EchoTestEvent::Error {
            error_code: _,
            error: _
        }))
    ));
}
