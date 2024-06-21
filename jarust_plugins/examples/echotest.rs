use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::transaction_gen::TransactionGenerationStrategy;
use jarust_plugins::echo_test::events::EchoTestEvent;
use jarust_plugins::echo_test::events::PluginEvent;
use jarust_plugins::echo_test::jahandle_ext::EchoTest;
use jarust_plugins::echo_test::msg_options::StartOptions;
use std::path::Path;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let filename = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let env_filter = EnvFilter::from_default_env()
        .add_directive("jarust=trace".parse()?)
        .add_directive(format!("{filename}=trace").parse()?);
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let config = JaConfig::builder().url("ws://localhost:8188/ws").build();
    let mut connection = jarust::connect(
        config,
        TransportType::Ws,
        TransactionGenerationStrategy::Random,
    )
    .await?;
    let session = connection.create(10, Duration::from_secs(10)).await?;
    let (handle, mut event_receiver) = session.attach_echo_test().await?;

    handle
        .start(StartOptions {
            audio: true,
            video: true,
            ..Default::default()
        })
        .await?;

    while let Some(event) = event_receiver.recv().await {
        match event {
            PluginEvent::EchoTestEvent(EchoTestEvent::Result { result, .. }) => {
                tracing::info!("result: {result}");
            }
            PluginEvent::EchoTestEvent(EchoTestEvent::ResultWithEstablishment {
                establishment_protocol,
                ..
            }) => {
                tracing::info!("establishment_protocol: {establishment_protocol:#?}");
            }
            PluginEvent::GenericEvent(event) => {
                tracing::debug!("generic event: {event:#?}");
            }
        }
    }

    Ok(())
}
