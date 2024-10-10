use jarust::jaconfig::JaConfig;
use jarust::jaconfig::JanusAPI;
use jarust::jaconnection::CreateConnectionParams;
use jarust_interface::japrotocol::EstablishmentProtocol;
use jarust_interface::japrotocol::Jsep;
use jarust_interface::japrotocol::JsepType;
use jarust_interface::tgenerator::RandomTransactionGenerator;
use jarust_plugins::video_room::jahandle_ext::VideoRoom;
use jarust_plugins::video_room::msg_options::*;
use jarust_plugins::JanusId;
use std::path::Path;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let filename = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let env_filter = EnvFilter::from_default_env()
        .add_directive("jarust=trace".parse()?)
        .add_directive("jarust_plugins=trace".parse()?)
        .add_directive("jarust_interface=trace".parse()?)
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
    let (handle, mut events) = session.attach_video_room(timeout).await?;

    tokio::spawn(async move {
        while let Some(e) = events.recv().await {
            tracing::info!("{e:#?}");
        }
    });

    let room_id = handle
        .create_room_with_config(
            VideoRoomCreateOptions {
                audiocodec: Some("opus".to_string()),
                videocodec: Some("h264".to_string()),
                notify_joining: Some(true),
                ..Default::default()
            },
            timeout,
        )
        .await?
        .room;

    handle
        .edit_room(
            room_id.clone(),
            VideoRoomEditOptions {
                new_description: Some("A brand new description!".to_string()),
                ..Default::default()
            },
            timeout,
        )
        .await?;

    let exists = handle.exists(room_id.clone(), timeout).await?;
    tracing::info!(
        "Does the room we just created and edited exist? {:#?}",
        exists.exists
    );

    let rooms = handle.list_rooms(timeout).await?;
    tracing::info!("Rooms {:#?}", rooms);

    let allowed_enable = handle
        .allowed(
            room_id.clone(),
            VideoRoomAllowedAction::Enable,
            vec![],
            Default::default(),
            timeout,
        )
        .await?;
    tracing::info!("Allowed list: {:#?}", allowed_enable.allowed);
    let allowed_add = handle
        .allowed(
            room_id.clone(),
            VideoRoomAllowedAction::Add,
            vec!["teststring".to_string(), "removeme".to_string()],
            Default::default(),
            timeout,
        )
        .await?;
    tracing::info!("Allowed list: {:#?}", allowed_add.allowed);
    let allowed_remove = handle
        .allowed(
            room_id.clone(),
            VideoRoomAllowedAction::Remove,
            vec!["removeme".to_string()],
            Default::default(),
            timeout,
        )
        .await?;
    tracing::info!("Allowed list: {:#?}", allowed_remove.allowed);
    handle
        .allowed(
            room_id.clone(),
            VideoRoomAllowedAction::Disable,
            vec![],
            Default::default(),
            timeout,
        )
        .await?;

    handle
        .join_as_publisher(
            room_id.clone(),
            VideoRoomPublisherJoinOptions {
                id: Some(JanusId::Uint(1337)),
                display: Some("xX1337-StreamerXx".into()),
                token: None,
            },
            None,
            timeout,
        )
        .await?;

    handle
        .publish(
            EstablishmentProtocol::JSEP(Jsep {
                jsep_type: JsepType::Offer,
                trickle: Some(false),
                sdp: EXAMPLE_SDP_OFFER.to_string(),
            }),
            VideoRoomPublishOptions {
                audiocodec: Some(VideoRoomAudioCodec::OPUS),
                videocodec: Some(VideoRoomVideoCodec::H264),
                bitrate: Some(3500),
                record: Some(false),
                descriptions: vec![VideoRoomPublishDescription {
                    mid: "stream-0".to_string(),
                    description: "The ultimate stream!!".to_string(),
                }],
                ..Default::default()
            },
            timeout,
        )
        .await?;

    let list_participants_rsp = handle.list_participants(room_id.clone(), timeout).await?;
    tracing::info!(
        "Participants in room {:#?}: {:#?}",
        list_participants_rsp.room,
        list_participants_rsp.participants
    );

    handle.unpublish(timeout).await?;

    handle.leave(timeout).await?;

    let list_participants_rsp = handle.list_participants(room_id.clone(), timeout).await?;
    tracing::info!(
        "Participants in room {:#?}: {:#?}",
        list_participants_rsp.room,
        list_participants_rsp.participants
    );

    handle
        .destroy_room(room_id, Default::default(), timeout)
        .await?;

    Ok(())
}

const EXAMPLE_SDP_OFFER: &str = "v=0
o=rtc 2683980088 0 IN IP4 127.0.0.1
s=-
t=0 0
a=group:BUNDLE 0 1
a=group:LS 0 1
a=msid-semantic:WMS *
a=setup:actpass
a=ice-ufrag:eBRl
a=ice-pwd:+AWJI4q7V5ivTpOnEyzoHL
a=ice-options:ice2,trickle
a=fingerprint:sha-256 00:6B:85:04:41:D1:AF:31:18:C5:32:43:E9:0D:17:D9:31:8A:01:89:10:B8:9D:05:06:14:DA:97:F4:E1:74:81
m=audio 63582 UDP/TLS/RTP/SAVPF 111
c=IN IP4 172.20.10.6
a=mid:0
a=sendonly
a=ssrc:2724817378 cname:20fq0G5qdxVf2T7D
a=ssrc:2724817378 msid:zwaqhEaMoL3k0x9g zwaqhEaMoL3k0x9g-audio
a=msid:zwaqhEaMoL3k0x9g zwaqhEaMoL3k0x9g-audio
a=rtcp-mux
a=rtpmap:111 opus/48000/2
a=fmtp:111 minptime=10;maxaveragebitrate=96000;stereo=1;sprop-stereo=1;useinbandfec=1
a=candidate:2 1 UDP 2130706175 2a00:20:c341:539e:c18:aed5:7682:7662 63582 typ host
a=candidate:1 1 UDP 2122317823 172.20.10.6 63582 typ host
a=candidate:3 1 UDP 2122317311 192.168.39.104 63582 typ host
a=end-of-candidates
m=video 63582 UDP/TLS/RTP/SAVPF 96
c=IN IP4 172.20.10.6
a=mid:1
a=sendonly
a=ssrc:2724817379 cname:20fq0G5qdxVf2T7D
a=ssrc:2724817379 msid:zwaqhEaMoL3k0x9g zwaqhEaMoL3k0x9g-video
a=msid:zwaqhEaMoL3k0x9g zwaqhEaMoL3k0x9g-video
a=rtcp-mux
a=rtpmap:96 H264/90000
a=rtcp-fb:96 nack
a=rtcp-fb:96 nack pli
a=rtcp-fb:96 goog-remb
a=fmtp:96 profile-level-id=42e01f;packetization-mode=1;level-asymmetry-allowed=1";
