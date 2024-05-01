use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::messages::CreateRoomMsg;
use jarust_plugins::audio_bridge::messages::DestroyRoomMsg;
use jarust_plugins::audio_bridge::AudioBridge;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("jarust=trace".parse()?))
        .init();
    let timeout = std::time::Duration::from_secs(10);

    let config = JaConfig::builder().url("ws://localhost:8188/ws").build();
    let mut connection = jarust::connect(config, TransportType::Ws).await?;
    let session = connection.create(10).await?;
    let (handle, ..) = session.attach_audio_bridge().await?;

    let exist = handle.exists(4321, timeout).await?;
    tracing::info!("Room exists?: {}", exist);

    if !exist {
        let _ = handle
            .create_room_with_config(
                CreateRoomMsg {
                    room: Some(4321),
                    secret: Some("superdupersecret".to_string()),
                    ..Default::default()
                },
                timeout,
            )
            .await?;

        let exist = handle.exists(4321, timeout).await?;
        tracing::info!("Room exists?: {}", exist);

        if exist {
            let _ = handle
                .destroy_room(
                    4321,
                    DestroyRoomMsg {
                        secret: Some("superdupersecret".to_string()),
                        ..Default::default()
                    },
                    timeout,
                )
                .await;

            let exist = handle.exists(4321, timeout).await?;
            tracing::info!("Room exists?: {}", exist);
        }
    }

    Ok(())
}
