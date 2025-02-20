use jarust::core::jaconfig::{JaConfig, JanusAPI};
use jarust::interface::tgenerator::RandomTransactionGenerator;
use jarust::plugins::video_room::events::PluginEvent;
use jarust::plugins::video_room::handle::VideoRoomHandle;
use jarust::plugins::video_room::jahandle_ext::VideoRoom;
use jarust::plugins::video_room::params::VideoRoomExistsParams;
use jarust::plugins::JanusId;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;

#[allow(unused_labels)]
#[tokio::test]
async fn room_crud_e2e() {
    let default_timeout = Duration::from_secs(4);
    let handle = make_videoroom_attachment().await.0;
    let room_id = JanusId::Uint(rand::random::<u64>().into());

    'before_creation: {
        let exists = handle
            .exists(
                VideoRoomExistsParams {
                    room: room_id.clone(),
                },
                default_timeout,
            )
            .await
            .expect("Failed to check if room exists; before_creation");
        assert!(!exists, "Room should not exist before creation");
    }
}

async fn make_videoroom_attachment() -> (VideoRoomHandle, UnboundedReceiver<PluginEvent>) {
    let config = JaConfig {
        url: "ws://localhost:8188/ws".to_string(),
        apisecret: None,
        server_root: "janus".to_string(),
        capacity: 32,
    };
    let mut connection =
        jarust::core::connect(config, JanusAPI::WebSocket, RandomTransactionGenerator)
            .await
            .expect("Failed to connect to server");
    let timeout = Duration::from_secs(10);
    let session = connection
        .create_session(10, Duration::from_secs(10))
        .await
        .expect("Failed to create session");
    let (handle, event_receiver) = session
        .attach_video_room(timeout)
        .await
        .expect("Failed to attach plugin");

    (handle, event_receiver)
}
