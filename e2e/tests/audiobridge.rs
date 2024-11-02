use jarust_core::jaconfig::JaConfig;
use jarust_core::jaconfig::JanusAPI;
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

// Allowing unused labels so we can use labels to have named blocks for test cases
// I feel it's better than comments
#[allow(unused_labels)]
#[tokio::test]
async fn room_crud_e2e() {
    let default_timeout = Duration::from_secs(4);
    let handle = make_audiobridge_attachment().await.0;
    let room_id = JanusId::Uint(rand::random());

    'before_creation: {
        let exists = handle
            .exists(
                AudioBridgeExistsParams {
                    room: room_id.clone(),
                },
                default_timeout,
            )
            .await
            .expect("Failed to check if room exists; before_creation");
        assert_eq!(exists, false, "Room should not exist before creation");
    }

    'creation: {
        handle
            .create_room(Some(room_id.clone()), default_timeout)
            .await
            .expect("Failed to create room; creation");
        let exists = handle
            .exists(
                AudioBridgeExistsParams {
                    room: room_id.clone(),
                },
                default_timeout,
            )
            .await
            .expect("Failed to check if room exists; creation");
        assert_eq!(exists, true, "Room should exist after creation");

        let rooms = handle
            .list_rooms(default_timeout)
            .await
            .expect("Failed to list rooms; creation");
        assert_eq!(
            rooms.iter().find(|room| room.room == room_id).is_some(),
            true,
            "Room should be visible when listing rooms"
        );
    }

    'description_edit: {
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
            .expect("Failed to edit room; description_edit");

        let rooms = handle
            .list_rooms(default_timeout)
            .await
            .expect("Failed to list rooms; description_edit");
        let edit_room = rooms
            .iter()
            .filter(|room| room.room == room_id)
            .next()
            .expect("Room not found; description_edit");
        assert_eq!(
            edit_room.description,
            "new description".to_string(),
            "Room description should be updated"
        );
    }

    'private_edit: {
        handle
            .edit_room(
                AudioBridgeEditParams {
                    room: room_id.clone(),
                    optional: AudioBridgeEditParamsOptional {
                        new_is_private: Some(true),
                        ..Default::default()
                    },
                },
                default_timeout,
            )
            .await
            .expect("Failed to edit room; private_edit");

        let rooms = handle
            .list_rooms(default_timeout)
            .await
            .expect("Failed to list rooms; private_edit");
        assert_eq!(
            rooms.iter().find(|room| room.room == room_id).is_some(),
            false,
            "Room should not be visible when listing rooms"
        );
        let exists = handle
            .exists(
                AudioBridgeExistsParams {
                    room: room_id.clone(),
                },
                default_timeout,
            )
            .await
            .expect("Failed to check if room exists; private_edit");
        assert_eq!(exists, true, "Room should exist after setting to private");
    }

    'destory: {
        handle
            .destroy_room(
                AudioBridgeDestoryParams {
                    room: room_id.clone(),
                    optional: Default::default(),
                },
                default_timeout,
            )
            .await
            .expect("Failed to destroy room; destory");
        let exists = handle
            .exists(
                AudioBridgeExistsParams {
                    room: room_id.clone(),
                },
                default_timeout,
            )
            .await
            .expect("Failed to check if room exists; destory");
        assert_eq!(exists, false, "Room should not exist after destruction");
    }
}

async fn make_audiobridge_attachment() -> (AudioBridgeHandle, UnboundedReceiver<PluginEvent>) {
    let config = JaConfig {
        url: "ws://localhost:8188/ws".to_string(),
        apisecret: None,
        server_root: "janus".to_string(),
        capacity: 32,
    };
    let mut connection =
        jarust_core::connect(config, JanusAPI::WebSocket, RandomTransactionGenerator)
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
