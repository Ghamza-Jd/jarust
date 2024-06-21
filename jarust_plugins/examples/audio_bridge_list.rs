use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::transaction_gen::TransactionGenerationStrategy;
use jarust_plugins::audio_bridge::jahandle_ext::AudioBridge;
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

    let config = JaConfig::builder().url("ws://localhost:8188/ws").build();
    let mut connection = jarust::connect(
        config,
        TransportType::Ws,
        TransactionGenerationStrategy::Random,
    )
    .await?;
    let session = connection.create(10, timeout).await?;
    let (handle, ..) = session.attach_audio_bridge().await?;

    let rooms = handle.list_rooms(timeout).await?;
    tracing::info!("Rooms {:#?}", rooms);

    Ok(())
}
