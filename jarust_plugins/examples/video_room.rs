use std::path::Path;

use tracing_subscriber::EnvFilter;

use jarust::jaconfig::{JaConfig, TransportType};
use jarust::params::CreateConnectionParams;
use jarust::TransactionGenerationStrategy;
use jarust_plugins::AttachPluginParams;
use jarust_plugins::video_room::jahandle_ext::VideoRoom;
use jarust_plugins::video_room::messages::{VideoRoomAllowedAction, VideoRoomEditOptions};

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
    let capacity = 10;
    let session = connection
        .create(CreateConnectionParams {
            ka_interval: 10,
            capacity,
            timeout,
        })
        .await?;
    let (handle, mut events) = session
        .attach_video_room(AttachPluginParams { capacity, timeout })
        .await?;

    let create_room_rsp = handle.create_room(None, timeout).await?;

    handle
        .edit_room(
            create_room_rsp.room,
            VideoRoomEditOptions {
                secret: None,
                new_description: Some("A brand new description!".to_string()),
                new_secret: None,
                new_pin: None,
                new_is_private: None,
                new_require_pvtid: None,
                new_bitrate: None,
                new_fir_freq: None,
                new_publishers: None,
                new_lock_record: None,
                new_rec_dir: None,
                permanent: None,
            },
            timeout,
        )
        .await?;

    let exists = handle.exists(create_room_rsp.room, timeout).await?;
    tracing::info!(
        "Does the room we just created and edited exist? {:#?}",
        exists.exists
    );

    let rooms = handle.list(timeout).await?;
    tracing::info!("Rooms {:#?}", rooms);

    let allowed_enable = handle
        .allowed(
            create_room_rsp.room,
            VideoRoomAllowedAction::Enable,
            vec![],
            Default::default(),
            timeout,
        )
        .await?;
    tracing::info!("Allowed list: {:#?}", allowed_enable.allowed);
    let allowed_add = handle
        .allowed(
            create_room_rsp.room,
            VideoRoomAllowedAction::Add,
            vec!["teststring".to_string(), "removeme".to_string()],
            Default::default(),
            timeout,
        )
        .await?;
    tracing::info!("Allowed list: {:#?}", allowed_add.allowed);
    let allowed_remove = handle
        .allowed(
            create_room_rsp.room,
            VideoRoomAllowedAction::Remove,
            vec!["removeme".to_string()],
            Default::default(),
            timeout,
        )
        .await?;
    tracing::info!("Allowed list: {:#?}", allowed_remove.allowed);
    handle
        .allowed(
            create_room_rsp.room,
            VideoRoomAllowedAction::Disable,
            vec![],
            Default::default(),
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

    handle
        .destroy_room(create_room_rsp.room, Default::default(), timeout)
        .await?;

    Ok(())
}
