use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::jahandle_ext::AudioBridge;
use std::path::Path;
use tokio::time;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let filename = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let env_filter = EnvFilter::from_default_env()
        .add_directive("jarust=trace".parse()?)
        .add_directive(format!("{filename}=trace").parse()?);
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
    let timeout = std::time::Duration::from_secs(10);

    let config = JaConfig::builder().url("ws://localhost:8188/ws").build();
    let mut connection = jarust::connect(config, TransportType::Ws).await?;
    let session = connection.create(10).await?;
    let (handle, ..) = session.attach_audio_bridge().await?;

    let first_room = handle
        .create_room_with_config(Default::default(), timeout)
        .await?;

    let second_room = handle
        .create_room_with_config(Default::default(), timeout)
        .await?;

    handle
        .join_room(first_room.room, Default::default(), None, timeout)
        .await?;

    handle
        .change_room(second_room.room, Default::default(), timeout)
        .await?;

    let mut interval = time::interval(time::Duration::from_secs(2));
    loop {
        interval.tick().await;
    }
}
