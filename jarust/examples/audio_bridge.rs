use jarust::core::connect;
use jarust::core::jaconfig::JaConfig;
use jarust::core::jaconfig::JanusAPI;
use jarust::interface::tgenerator::RandomTransactionGenerator;
use jarust::plugins::audio_bridge::jahandle_ext::AudioBridge;
use jarust::plugins::audio_bridge::params::AudioBridgeJoinParams;
use jarust::plugins::audio_bridge::params::AudioBridgeJoinParamsOptional;
use jarust::plugins::audio_bridge::params::AudioBridgeListParticipantsParams;
use jarust::plugins::audio_bridge::params::AudioBridgeMuteParams;
use std::path::Path;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let filename = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let env_filter = EnvFilter::from_default_env()
        .add_directive("jarust_core=trace".parse()?)
        .add_directive("jarust_plugins=trace".parse()?)
        .add_directive("jarust_interface=trace".parse()?)
        .add_directive("jarust_rt=trace".parse()?)
        .add_directive(format!("{filename}=trace").parse()?);
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let timeout = Duration::from_secs(10);
    let config = JaConfig {
        url: "ws://localhost:8188/ws".to_string(),
        apisecret: None,
        server_root: "janus".to_string(),
        capacity: 32,
    };
    let mut connection = connect(config, JanusAPI::WebSocket, RandomTransactionGenerator).await?;
    let session = connection
        .create_session(10, Duration::from_secs(10))
        .await?;
    let (handle, mut events) = session.attach_audio_bridge(timeout).await?;

    let create_room_rsp = handle.create_room(None, timeout).await?;
    // Try create a room that already exist
    let error_room = handle
        .create_room(Some(create_room_rsp.room.clone()), timeout)
        .await;
    if let Err(jarust::interface::Error::PluginResponseError { .. }) = error_room {
        tracing::info!("Already created");
    };

    let rooms = handle.list_rooms(timeout).await?;

    tracing::info!("Rooms {:#?}", rooms);

    let join_room_params = AudioBridgeJoinParams {
        room: create_room_rsp.room.clone(),
        optional: AudioBridgeJoinParamsOptional {
            display: Some("value".to_string()),
            ..Default::default()
        },
    };

    handle.join_room(join_room_params, None, timeout).await?;

    let list_participants_rsp = handle
        .list_participants(
            AudioBridgeListParticipantsParams {
                room: create_room_rsp.room,
            },
            timeout,
        )
        .await?;
    tracing::info!(
        "Participants in room {:#?}: {:#?}",
        list_participants_rsp.room,
        list_participants_rsp.participants
    );

    use jarust::plugins::audio_bridge::events::AudioBridgeEvent as ABE;
    use jarust::plugins::audio_bridge::events::PluginEvent as PE;
    if let Some(PE::AudioBridgeEvent(ABE::RoomJoined { id, room, .. })) = events.recv().await {
        handle
            .mute(AudioBridgeMuteParams {
                id: id.clone(),
                room: room.clone(),
                secret: None,
            })
            .await?;

        handle
            .unmute(AudioBridgeMuteParams {
                id: id.clone(),
                room: room.clone(),
                secret: None,
            })
            .await?;
    };

    while let Some(e) = events.recv().await {
        tracing::info!("{e:#?}");
    }

    Ok(())
}
