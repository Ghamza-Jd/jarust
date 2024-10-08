use jarust::jaconfig::JaConfig;
use jarust::jaconfig::JanusAPI;
use jarust::jaconnection::CreateConnectionParams;
use jarust_interface::tgenerator::RandomTransactionGenerator;
use jarust_plugins::streaming::jahandle_ext::Streaming;
use jarust_plugins::streaming::msg_options::*;
use jarust_plugins::JanusId;
use std::path::Path;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let filename = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let env_filter = EnvFilter::from_default_env()
        .add_directive("jarust=trace".parse()?)
        .add_directive("jarust_plugins=trace".parse()?)
        .add_directive("jarust_transport=trace".parse()?)
        .add_directive("jarust_rt=trace".parse()?)
        .add_directive(format!("{filename}=trace").parse()?);
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let timeout = std::time::Duration::from_secs(10);
    let config = JaConfig::builder()
        .url("ws://localhost:8188/ws")
        .capacity(32)
        .build();
    let mut connection =
        jarust::connect(config, JanusAPI::WebSocket, RandomTransactionGenerator).await?;
    let session = connection
        .create_session(CreateConnectionParams {
            ka_interval: 10,
            timeout,
        })
        .await?;
    let (handle, mut events) = session.attach_streaming(timeout).await?;

    tokio::spawn(async move {
        while let Some(e) = events.recv().await {
            tracing::info!("{e:#?}");
        }
    });

    let mountpoint_id = handle
        .create_mountpoint_with_config(
            StreamingCreateOptions {
                id: Some(JanusId::Uint(1337)),
                name: Some("stream name".to_string()),
                description: Some("stream description".to_string()),
                mountpoint_type: "rtp".to_string(),
                media: Some(Vec::from([StreamingRtpMedia {
                    media_type: "video".to_string(),
                    mid: "v".to_string(),
                    port: 0,
                    pt: Some(100),
                    codec: Some("vp8".to_string()),
                    ..Default::default()
                }])),
                ..Default::default()
            },
            timeout,
        )
        .await?
        .stream
        .id;

    let mountpoints = handle.list(timeout).await?;
    tracing::info!("Mountpoints {:#?}", mountpoints);

    let info = handle.info(JanusId::Uint(1337), None, timeout).await?;
    tracing::info!("Info: {:#?}", info);

    handle
        .destroy_mountpoint(mountpoint_id, Default::default(), timeout)
        .await?;

    Ok(())
}
