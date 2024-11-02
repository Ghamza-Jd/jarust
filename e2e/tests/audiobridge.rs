use jarust::jaconfig::JaConfig;
use jarust::jaconfig::JanusAPI;
use jarust_interface::tgenerator::RandomTransactionGenerator;
use jarust_plugins::audio_bridge::events::PluginEvent;
use jarust_plugins::audio_bridge::handle::AudioBridgeHandle;
use jarust_plugins::audio_bridge::jahandle_ext::AudioBridge;
use jarust_plugins::audio_bridge::params::AudioBridgeDestoryParams;
use jarust_plugins::audio_bridge::params::AudioBridgeEditParams;
use jarust_plugins::audio_bridge::params::AudioBridgeEditParamsOptional;
use jarust_plugins::audio_bridge::params::AudioBridgeExistsParams;
use jarust_plugins::JanusId;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;

#[tokio::test]
async fn room_crud_e2e() {
    let default_timeout = Duration::from_secs(4);
    let handle = make_audiobridge_attachment().await.0;

    let room_id = JanusId::Uint(rand::random());

    let exists = handle
        .exists(
            AudioBridgeExistsParams {
                room: room_id.clone(),
            },
            default_timeout,
        )
        .await
        .expect("Failed to check if room exists");
    assert_eq!(exists, false);

    handle
        .create_room(Some(room_id.clone()), default_timeout)
        .await
        .expect("Failed to create room");
    let exists = handle
        .exists(
            AudioBridgeExistsParams {
                room: room_id.clone(),
            },
            default_timeout,
        )
        .await
        .expect("Failed to check if room exists");
    assert_eq!(exists, true);

    handle
        .edit_room(
            AudioBridgeEditParams {
                room: room_id.clone(),
                optional: AudioBridgeEditParamsOptional {
                    new_description: Some("new description".to_string()),
                    ..Default::default()
                },
            },
            default_timeout,
        )
        .await
        .expect("Failed to edit room");

    let rooms = handle
        .list_rooms(default_timeout)
        .await
        .expect("Failed to list rooms");
    let edit_room = rooms
        .iter()
        .filter(|room| room.room == room_id)
        .next()
        .expect("Room not found");
    assert_eq!(edit_room.description, "new description".to_string());

    handle
        .destroy_room(
            AudioBridgeDestoryParams {
                room: room_id.clone(),
                optional: Default::default(),
            },
            default_timeout,
        )
        .await
        .expect("Failed to destroy room");
    let exists = handle
        .exists(
            AudioBridgeExistsParams {
                room: room_id.clone(),
            },
            default_timeout,
        )
        .await
        .expect("Failed to check if room exists");
    assert_eq!(exists, false);
}

async fn make_audiobridge_attachment() -> (AudioBridgeHandle, UnboundedReceiver<PluginEvent>) {
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
    let (handle, event_receiver) = session
        .attach_audio_bridge(timeout)
        .await
        .expect("Failed to attach plugin");

    (handle, event_receiver)
}
