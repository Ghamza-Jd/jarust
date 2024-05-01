use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::echotest::events::EchoTestPluginEvent;
use jarust_plugins::echotest::jahandle_ext::EchoTest;
use jarust_plugins::echotest::messages::StartMsg;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::from_default_env()
        .add_directive("jarust=trace".parse()?)
        .add_directive("echotest=trace".parse()?);
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let config = JaConfig::builder().url("ws://localhost:8188/ws").build();
    let mut connection = jarust::connect(config, TransportType::Ws).await?;
    let session = connection.create(10).await?;
    let (handle, mut event_receiver, ..) = session.attach_echotest().await?;

    handle
        .start(StartMsg {
            audio: true,
            video: true,
            ..Default::default()
        })
        .await?;

    while let Some(event) = event_receiver.recv().await {
        match event {
            EchoTestPluginEvent::Result { result, .. } => {
                tracing::info!("result: {result}");
            }
        }
    }

    Ok(())
}
