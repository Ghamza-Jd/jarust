use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::TransactionGenerationStrategy;
use jarust_plugins::audio_bridge::common::Identifier;
use jarust_plugins::audio_bridge::jahandle_ext::AudioBridge;
use jarust_plugins::audio_bridge::msg_opitons::CreateRoomOptions;
use jarust_plugins::audio_bridge::msg_opitons::EditRoomOptions;
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
    let (handle, ..) = session.attach_audio_bridge().await?;

    let _ = handle
        .create_room_with_config(
            CreateRoomOptions {
                room: Some(Identifier::Uint(4321)),
                description: Some("A nice description".to_string()),
                secret: Some("superdupersecret".to_string()),
                ..Default::default()
            },
            timeout,
        )
        .await?;

    let edit_room_rsp = handle
        .edit_room(
            Identifier::Uint(4321),
            EditRoomOptions {
                new_description: Some("A nicer description".to_string()),
                secret: Some("superdupersecret".to_string()),
                ..Default::default()
            },
            timeout,
        )
        .await?;

    tracing::info!("Edited Room {:#?}", edit_room_rsp.room);

    Ok(())
}
