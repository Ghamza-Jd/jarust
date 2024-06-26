use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::params::CreateConnectionParams;
use jarust::TransactionGenerationStrategy;
use jarust_plugins::audio_bridge::jahandle_ext::AudioBridge;
use jarust_plugins::audio_bridge::msg_opitons::MuteOptions;
use jarust_plugins::AttachPluginParams;
use std::path::Path;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let filename = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let env_filter = EnvFilter::from_default_env()
        .add_directive("jarust=trace".parse()?)
        .add_directive(format!("{filename}=trace").parse()?);
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let timeout = std::time::Duration::from_secs(10);
    let config = JaConfig::builder()
        .url("ws://localhost:8188/ws")
        .capacity(
            32, /* Buffer size on the entire connection with janus */
        )
        .build();
    let mut connection = jarust::connect(
        config,
        TransportType::Ws,
        TransactionGenerationStrategy::Random,
    )
    .await?;
    let capacity = 10;
    let session = connection
        .create(CreateConnectionParams {
            ka_interval: 10,
            capacity,
            timeout,
        })
        .await?;
    let (handle, mut events) = session
        .attach_audio_bridge(AttachPluginParams { capacity, timeout })
        .await?;

    let create_room_rsp = handle.create_room(None, timeout).await?;
    let rooms = handle.list_rooms(timeout).await?;

    tracing::info!("Rooms {:#?}", rooms);

    handle
        .join_room(
            create_room_rsp.room.clone(),
            Default::default(),
            None,
            timeout,
        )
        .await?;

    let list_participants_rsp = handle
        .list_participants(create_room_rsp.room, timeout)
        .await?;
    tracing::info!(
        "Participants in room {:#?}: {:#?}",
        list_participants_rsp.room,
        list_participants_rsp.participants
    );

    use jarust_plugins::audio_bridge::events::AudioBridgeEvent as ABE;
    use jarust_plugins::audio_bridge::events::PluginEvent as PE;
    if let Some(PE::AudioBridgeEvent(ABE::RoomJoined { id, room, .. })) = events.recv().await {
        handle
            .mute(MuteOptions {
                id: id.clone(),
                room: room.clone(),
                secret: None,
            })
            .await?;

        handle
            .unmute(MuteOptions {
                id,
                room,
                secret: None,
            })
            .await?;
    };

    while let Some(e) = events.recv().await {
        tracing::info!("{e:#?}");
    }

    Ok(())
}
