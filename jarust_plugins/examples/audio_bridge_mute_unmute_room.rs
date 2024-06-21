use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::TransactionGenerationStrategy;
use jarust_plugins::audio_bridge::jahandle_ext::AudioBridge;
use jarust_plugins::audio_bridge::msg_opitons::MuteRoomOptions;
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
        .capacity(32)
        .build();
    let mut connection = jarust::connect(
        config,
        TransportType::Ws,
        TransactionGenerationStrategy::Random,
    )
    .await?;
    let session = connection.create(10, timeout).await?;
    let (handle, mut events) = session.attach_audio_bridge().await?;

    let create_room_rsp = handle.create_room(None, timeout).await.unwrap();
    handle
        .join_room(create_room_rsp.room, Default::default(), None, timeout)
        .await?;

    use jarust_plugins::audio_bridge::events::AudioBridgeEvent as ABE;
    use jarust_plugins::audio_bridge::events::PluginEvent as PE;
    if let Some(PE::AudioBridgeEvent(ABE::RoomJoined { room, .. })) = events.recv().await {
        handle
            .mute_room(MuteRoomOptions {
                room: room.clone(),
                secret: None,
            })
            .await?;

        handle
            .unmute_room(MuteRoomOptions { room, secret: None })
            .await?;
    };

    while let Some(e) = events.recv().await {
        tracing::info!("{e:#?}");
    }

    Ok(())
}
