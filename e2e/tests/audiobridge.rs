use jarust::core::jaconfig::JaConfig;
use jarust::core::jaconfig::JanusAPI;
use jarust::interface::tgenerator::RandomTransactionGenerator;
use jarust::plugins::audio_bridge::common::AudioBridgeParticipant;
use jarust::plugins::audio_bridge::events::AudioBridgeEvent;
use jarust::plugins::audio_bridge::events::PluginEvent;
use jarust::plugins::audio_bridge::handle::AudioBridgeHandle;
use jarust::plugins::audio_bridge::jahandle_ext::AudioBridge;
use jarust::plugins::audio_bridge::params::AudioBridgeDestroyParams;
use jarust::plugins::audio_bridge::params::AudioBridgeEditParams;
use jarust::plugins::audio_bridge::params::AudioBridgeEditParamsOptional;
use jarust::plugins::audio_bridge::params::AudioBridgeExistsParams;
use jarust::plugins::audio_bridge::params::AudioBridgeJoinParams;
use jarust::plugins::audio_bridge::params::AudioBridgeMuteParams;
use jarust::plugins::JanusId;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;

// Allowing unused labels so we can use labels to have named blocks for test cases
// I feel it's better than comments
#[allow(unused_labels)]
#[tokio::test]
async fn room_crud_e2e() {
    let default_timeout = Duration::from_secs(4);
    let handle = make_audiobridge_attachment().await.0;
    let room_id = JanusId::Uint(rand::random::<u64>().into());

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
        assert!(!exists, "Room should not exist before creation");
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
        assert!(exists, "Room should exist after creation");

        let rooms = handle
            .list_rooms(default_timeout)
            .await
            .expect("Failed to list rooms; creation");
        assert!(
            rooms.iter().any(|room| room.room == room_id),
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
            .find(|room| room.room == room_id)
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
        assert!(
            !rooms.iter().any(|room| room.room == room_id),
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
        assert!(exists, "Room should exist after setting to private");
    }

    'destroy: {
        handle
            .destroy_room(
                AudioBridgeDestroyParams {
                    room: room_id.clone(),
                    optional: Default::default(),
                },
                default_timeout,
            )
            .await
            .expect("Failed to destroy room; destroy");
        let exists = handle
            .exists(
                AudioBridgeExistsParams {
                    room: room_id.clone(),
                },
                default_timeout,
            )
            .await
            .expect("Failed to check if room exists; destroy");
        assert!(!exists, "Room should not exist after destruction");
    }
}

#[allow(unused_labels)]
#[tokio::test]
async fn participants_e2e() {
    let default_timeout = Duration::from_secs(4);
    let room_id = JanusId::Uint(rand::random::<u64>().into());
    let admin = make_audiobridge_attachment().await.0;
    let (alice_handle, mut alice_events) = make_audiobridge_attachment().await;
    let (bob_handle, mut bob_events) = make_audiobridge_attachment().await;
    let (eve_handle, mut eve_events) = make_audiobridge_attachment().await;

    admin
        .create_room(Some(room_id.clone()), default_timeout)
        .await
        .expect("Admin failed to create room; creation");

    // Alice joins the room
    let alice = {
        alice_handle
            .join_room(
                AudioBridgeJoinParams {
                    room: room_id.clone(),
                    optional: Default::default(),
                },
                None,
                default_timeout,
            )
            .await
            .expect("Alice failed to join room");

        let PluginEvent::AudioBridgeEvent(AudioBridgeEvent::RoomJoined {
            participants,
            room,
            id,
        }) = alice_events
            .recv()
            .await
            .expect("Eve failed to receive event")
        else {
            panic!("Eve received unexpected event")
        };

        assert_eq!(room, room_id, "Alice should join correct room");
        assert_eq!(participants, vec![], "No participants should be in room");

        AudioBridgeParticipant {
            id,
            display: None,
            setup: false,
            muted: false,
            suspended: None,
            talking: None,
            spatial_position: None,
        }
    };

    // Bob joins the room
    let _bob = {
        bob_handle
            .join_room(
                AudioBridgeJoinParams {
                    room: room_id.clone(),
                    optional: Default::default(),
                },
                None,
                default_timeout,
            )
            .await
            .expect("Bob failed to join room");

        let PluginEvent::AudioBridgeEvent(AudioBridgeEvent::RoomJoined {
            participants,
            room,
            id,
        }) = bob_events
            .recv()
            .await
            .expect("Eve failed to receive event")
        else {
            panic!("Eve received unexpected event")
        };

        assert_eq!(room, room_id, "Bob should join correct room");
        assert_eq!(
            participants,
            vec![alice.clone()],
            "Only Alice should be in room"
        );

        AudioBridgeParticipant {
            id,
            display: None,
            setup: false,
            muted: false,
            suspended: None,
            talking: None,
            spatial_position: None,
        }
    };

    // Eve joins the room
    let _eve = {
        eve_handle
            .join_room(
                AudioBridgeJoinParams {
                    room: room_id.clone(),
                    optional: Default::default(),
                },
                None,
                default_timeout,
            )
            .await
            .expect("Eve failed to join room");

        let PluginEvent::AudioBridgeEvent(AudioBridgeEvent::RoomJoined {
            participants,
            room,
            id,
        }) = eve_events
            .recv()
            .await
            .expect("Eve failed to receive event")
        else {
            panic!("Eve received unexpected event")
        };

        assert_eq!(room, room_id, "Eve should join correct room");
        assert_eq!(participants.len(), 2, "Alice and Bob should be in room");
        assert_eq!(
            participants.contains(&alice),
            true,
            "Alice should be in room"
        );

        AudioBridgeParticipant {
            id,
            display: None,
            setup: false,
            muted: false,
            suspended: None,
            talking: None,
            spatial_position: None,
        }
    };

    'mute: {
        eve_handle
            .mute(AudioBridgeMuteParams {
                room: room_id.clone(),
                id: alice.id.clone(),
                secret: None,
            })
            .await
            .expect("Failed to mute participant; mute_and_unmute");

        // Alice should receive the mute event of herself
        let PluginEvent::AudioBridgeEvent(AudioBridgeEvent::ParticipantsUpdated {
            participants,
            ..
        }) = alice_events
            .recv()
            .await
            .expect("Alice failed to receive event")
        else {
            panic!("Alice received unexpected event")
        };

        assert_eq!(
            participants
                .iter()
                .find(|p| p.id == alice.id)
                .expect("Alice not found")
                .muted,
            true
        );

        // Bob should receive the mute event of Alice
        let PluginEvent::AudioBridgeEvent(AudioBridgeEvent::ParticipantsUpdated {
            participants,
            ..
        }) = bob_events
            .recv()
            .await
            .expect("Bob failed to receive event")
        else {
            panic!("Bob received unexpected event")
        };

        assert_eq!(
            participants
                .iter()
                .find(|p| p.id == alice.id)
                .expect("Alice not found")
                .muted,
            true
        );

        // Eve should receive the mute event of Alice
        let PluginEvent::AudioBridgeEvent(AudioBridgeEvent::ParticipantsUpdated {
            participants,
            ..
        }) = eve_events
            .recv()
            .await
            .expect("Eve failed to receive event")
        else {
            panic!("Eve received unexpected event")
        };

        assert_eq!(
            participants
                .iter()
                .find(|p| p.id == alice.id)
                .expect("Alice not found")
                .muted,
            true
        );
    }

    'unmute: {
        eve_handle
            .unmute(AudioBridgeMuteParams {
                room: room_id.clone(),
                id: alice.id.clone(),
                secret: None,
            })
            .await
            .expect("Failed to unmute participant; mute_and_unmute");

        // Alice should receive the unmute event of herself
        let PluginEvent::AudioBridgeEvent(AudioBridgeEvent::ParticipantsUpdated {
            participants,
            ..
        }) = alice_events
            .recv()
            .await
            .expect("Alice failed to receive event")
        else {
            panic!("Alice received unexpected event")
        };

        assert_eq!(
            participants
                .iter()
                .find(|p| p.id == alice.id)
                .expect("Alice not found")
                .muted,
            false
        );

        // Bob should receive the unmute event of Alice
        let PluginEvent::AudioBridgeEvent(AudioBridgeEvent::ParticipantsUpdated {
            participants,
            ..
        }) = bob_events
            .recv()
            .await
            .expect("Bob failed to receive event")
        else {
            panic!("Bob received unexpected event")
        };

        assert_eq!(
            participants
                .iter()
                .find(|p| p.id == alice.id)
                .expect("Alice not found")
                .muted,
            false
        );

        // Eve should receive the unmute event of Alice
        let PluginEvent::AudioBridgeEvent(AudioBridgeEvent::ParticipantsUpdated {
            participants,
            ..
        }) = eve_events
            .recv()
            .await
            .expect("Eve failed to receive event")
        else {
            panic!("Eve received unexpected event")
        };

        assert_eq!(
            participants
                .iter()
                .find(|p| p.id == alice.id)
                .expect("Alice not found")
                .muted,
            false
        );
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
        jarust::core::connect(config, JanusAPI::WebSocket, RandomTransactionGenerator)
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
