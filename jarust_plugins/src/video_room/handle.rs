use crate::video_room::msg_options::*;
use crate::video_room::responses::*;
use crate::JanusId;
use jarust::prelude::*;
use jarust_interface::japrotocol::EstablishmentProtocol;
use jarust_rt::JaTask;
use serde_json::json;
use serde_json::Value;
use std::ops::Deref;
use std::time::Duration;

pub struct VideoRoomHandle {
    handle: JaHandle,
    task: Option<JaTask>,
}

//
// synchronous methods
//
impl VideoRoomHandle {
    /// Create a new video room dynamically with the given room number,
    /// as an alternative to using the configuration file
    ///
    /// ### Note:
    /// Random room number will be used if `room` is `None`
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn create_room(
        &self,
        room: Option<JanusId>,
        timeout: Duration,
    ) -> Result<RoomCreatedRsp, jarust_interface::Error> {
        self.create_room_with_config(
            VideoRoomCreateOptions {
                room,
                ..Default::default()
            },
            timeout,
        )
        .await
    }

    /// Create a new audio room dynamically with the given configuration,
    /// as an alternative to using the configuration file
    ///
    /// ### Note:
    /// Random room number will be used if `room` is `None`
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn create_room_with_config(
        &self,
        options: VideoRoomCreateOptions,
        timeout: Duration,
    ) -> Result<RoomCreatedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "videoroom", "Sending create");
        let mut message: Value = options.try_into()?;
        message["request"] = "create".into();

        self.handle
            .send_waiton_rsp::<RoomCreatedRsp>(message, timeout)
            .await
    }

    // Destroy an existing video room, whether created dynamically or statically
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn destroy_room(
        &self,
        options: VideoRoomDestroyOptions,
        timeout: Duration,
    ) -> Result<RoomDestroyedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "videoroom", "Sending destroy");
        let mut message: Value = options.try_into()?;
        message["request"] = "destroy".into();

        self.handle
            .send_waiton_rsp::<RoomDestroyedRsp>(message, timeout)
            .await
    }

    /// Allows you to dynamically edit some room properties (e.g., the PIN)
    ///
    /// ### Note:
    /// You won't be able to modify other more static properties,
    /// like the room ID, the sampling rate, the extensions-related stuff and so on.
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn edit_room(
        &self,
        options: VideoRoomEditOptions,
        timeout: Duration,
    ) -> Result<RoomEditedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "videoroom", "Sending edit");
        let mut message: Value = options.try_into()?;
        message["request"] = "edit".into();

        self.handle
            .send_waiton_rsp::<RoomEditedRsp>(message, timeout)
            .await
    }

    /// Check whether a room exists
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn exists(
        &self,
        room: JanusId,
        timeout: Duration,
    ) -> Result<RoomExistsRsp, jarust_interface::Error> {
        tracing::info!(plugin = "videoroom", "Sending exists");
        let message = json!({
            "request": "exists",
            "room": room
        });

        self.handle
            .send_waiton_rsp::<RoomExistsRsp>(message, timeout)
            .await
    }

    /// Get a list of the available rooms
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn list_rooms(
        &self,
        timeout: Duration,
    ) -> Result<Vec<Room>, jarust_interface::Error> {
        tracing::info!(plugin = "videoroom", "Sending list");
        let response = self
            .handle
            .send_waiton_rsp::<ListRoomsRsp>(json!({"request": "list"}), timeout)
            .await?;

        Ok(response.list)
    }

    /// Allows you to edit who's allowed to join a room via ad-hoc tokens
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn allowed(
        &self,
        options: VideoRoomAllowedOptions,
        timeout: Duration,
    ) -> Result<AccessRsp, jarust_interface::Error> {
        if (options.action == VideoRoomAllowedAction::Enable
            || options.action == VideoRoomAllowedAction::Disable)
            && !options.allowed.is_empty()
        {
            return Err(jarust_interface::Error::InvalidJanusRequest {
                reason: "An enable or disable 'allowed' request cannot have its allowed array set"
                    .to_string(),
            });
        }

        tracing::info!(plugin = "videoroom", "Sending allowed");
        let mut message: Value = options.try_into()?;
        message["request"] = "allowed".into();

        self.handle
            .send_waiton_rsp::<AccessRsp>(message, timeout)
            .await
    }

    /// Kicks a participants out of a room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn kick(
        &self,
        options: VideoRoomKickOptions,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "videoroom", "Sending kick");
        let mut message: Value = options.try_into()?;
        message["request"] = "kick".into();

        self.handle.send_waiton_rsp::<()>(message, timeout).await
    }

    /// Enable or disable recording on all participants while the conference is in progress
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn enable_recording(
        &self,
        options: VideoRoomEnableRecordingOptions,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        tracing::info!(plugin = "videoroom", "Sending enable recording");
        let mut message: Value = options.try_into()?;
        message["request"] = "enable_recording".into();

        self.handle.send_waiton_rsp::<()>(message, timeout).await
    }

    /// Get a list of the participants in a specific room
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn list_participants(
        &self,
        room: JanusId,
        timeout: Duration,
    ) -> Result<ListParticipantsRsp, jarust_interface::Error> {
        tracing::info!(plugin = "videoroom", "Sending list participants");
        self.handle
            .send_waiton_rsp::<ListParticipantsRsp>(
                json!({"request": "listparticipants", "room": room }),
                timeout,
            )
            .await
    }

    #[cfg(feature = "__experimental")]
    pub async fn moderate(
        &self,
        options: VideoRoomModerateOptions,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "moderate".into();

        self.handle.send_waiton_rsp::<()>(message, timeout).await
    }

    #[cfg(feature = "__experimental")]
    pub async fn list_forwarders(
        &self,
        options: VideoRoomListForwardersOptions,
        timeout: Duration,
    ) -> Result<ListForwardersRsp, jarust_interface::Error> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "list_forwarders".into();

        self.handle
            .send_waiton_rsp::<ListForwardersRsp>(message, timeout)
            .await
    }

    #[cfg(feature = "__experimental")]
    pub async fn rtp_forward(
        &self,
        options: VideoRoomRtpForwardOptions,
        timeout: Duration,
    ) -> Result<RtpForwardRsp, jarust_interface::Error> {
        let mut message = serde_json::to_value(options)?;
        message["request"] = "rtp_forward".into();

        self.handle
            .send_waiton_rsp::<RtpForwardRsp>(message, timeout)
            .await
    }

    #[cfg(feature = "__experimental")]
    pub async fn stop_rtp_forward(
        &self,
        room: JanusId,
        publisher_id: JanusId,
        stream_id: u64,
        timeout: Duration,
    ) -> Result<StopRtpForwardRsp, jarust_interface::Error> {
        self.handle
            .send_waiton_rsp::<StopRtpForwardRsp>(
                json!({
                    "request": "stop_rtp_forward",
                    "room": room,
                    "publisher_id": publisher_id,
                    "stream_id": stream_id
                }),
                timeout,
            )
            .await
    }
}

//
// asynchronous methods
//
impl VideoRoomHandle {
    /// Join a room as a publishers
    ///
    /// In a VideoRoom, publishers are those participant handles that are able (although may choose not to)
    /// publish media in the room, and as such become feeds that you can subscribe to.
    /// To specify that a handle will be associated with a publisher, you must use the `join_as_publisher` request
    /// (note that you can also use [`VideoRoomHandle::join_and_configure`] for the purpose).
    ///
    /// A successful join will result in a [`VideoRoomEvent::RoomJoined`](super::events::VideoRoomEvent::RoomJoined) event,
    /// which will contain a list of the currently active (as in publishing via WebRTC) publishers,
    /// and optionally a list of passive attendees (but only if the room was configured with notify_joining set to TRUE)
    pub async fn join_as_publisher(
        &self,
        options: VideoRoomPublisherJoinOptions,
        protocol: Option<EstablishmentProtocol>,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        let mut message: Value = options.try_into()?;
        message["request"] = "join".into();
        message["ptype"] = "publisher".into();

        match protocol {
            None => self.handle.send_waiton_ack(message, timeout).await?,
            Some(ep) => {
                self.handle
                    .send_waiton_ack_with_est(message, ep, timeout)
                    .await?
            }
        };
        Ok(())
    }

    /// Join a room as a subscriber
    ///
    /// In a VideoRoom, subscribers are NOT participants, but simply handles that will be used exclusively to
    /// receive media from one or more publishers in the room. Since they're not participants per se,
    /// they're basically streams that can be (and typically are) associated to publisher handles
    /// as the ones we introduced in the previous section, whether active or not.
    /// In fact, the typical use case is publishers being notified about new participants becoming active in the room,
    /// and as a result new subscriber sessions being created to receive their media streams;
    /// as soon as the publisher goes away, other participants are notified so that the related subscriber handles
    /// can be removed/updated accordingly as well. As such, these subscriber sessions are dependent on feedback
    /// obtained by publishers, and can't exist on their own, unless you feed them the right info out of band
    /// (which is impossible in rooms configured with require_pvtid).
    pub async fn join_as_subscriber(
        &self,
        options: VideoRoomSubscriberJoinOptions,
        protocol: Option<EstablishmentProtocol>,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        let mut message: Value = options.try_into()?;
        message["request"] = "join".into();
        message["ptype"] = "subscriber".into();

        match protocol {
            None => self.handle.send_waiton_ack(message, timeout).await?,
            Some(ep) => {
                self.handle
                    .send_waiton_ack_with_est(message, ep, timeout)
                    .await?
            }
        };
        Ok(())
    }

    /// Tweak some of the properties of an active publisher session
    ///
    /// It's basically the same properties as those listed for publish , with the addition of a `streams` array that can be used
    /// to tweak individual streams (which is not available when publishing since in that case the stream doesn't exist yet).
    /// Notice that the configure request can also be used in renegotiations, to provide an updated SDP with changes to the published media.
    pub async fn configure_publisher(
        &self,
        options: VideoRoomConfigurePublisherOptions,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        let mut message: Value = options.try_into()?;
        message["request"] = "configure".into();
        self.handle.send_waiton_ack(message, timeout).await?;
        Ok(())
    }

    /// This request allows subscribers to dynamically change some properties associated to their media subscription,
    /// e.g., in terms of what should and should not be sent at a specific time.
    pub async fn configure_subscriber(
        &self,
        options: VideoRoomConfigureSubscriberOptions,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        let mut message: Value = options.try_into()?;
        message["request"] = "configure".into();
        self.handle.send_waiton_ack(message, timeout).await?;
        Ok(())
    }

    /// A combination of [VideoRoomHandle::join_as_publisher()] and [VideoRoomHandle::configure_publisher()]
    pub async fn join_and_configure(
        &self,
        join_options: VideoRoomPublisherJoinOptions,
        configure_options: VideoRoomConfigurePublisherOptions,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        let mut message: Value = JoinAndConfigureOptions {
            join_options,
            configure_options,
        }
        .try_into()?;
        message["request"] = "joinandconfigure".into();
        self.handle.send_waiton_ack(message, timeout).await?;
        Ok(())
    }

    /// Start publishing in a room
    ///
    /// This request MUST be accompanied by a JSEP SDP offer to negotiate a new PeerConnection.
    /// The plugin will match it to the room configuration (e.g., to make sure the codecs you negotiated are allowed in the room),
    /// and will reply with a JSEP SDP answer to close the circle and complete the setup of the PeerConnection.
    /// As soon as the PeerConnection has been established, the publisher will become active, and a new active feed other participants can subscribe to.
    pub async fn publish(
        &self,
        options: VideoRoomPublishOptions,
        establishment_protocol: EstablishmentProtocol,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        let mut message: Value = options.try_into()?;
        message["request"] = "publish".into();
        self.handle
            .send_waiton_ack_with_est(message, establishment_protocol, timeout)
            .await?;
        Ok(())
    }

    /// Stop publishing and tear down the related PeerConnection
    ///
    /// This request requires no arguments as the context is implicit.
    pub async fn unpublish(&self, timeout: Duration) -> Result<(), jarust_interface::Error> {
        self.handle
            .send_waiton_ack(json!({"request": "unpublish"}), timeout)
            .await?;
        Ok(())
    }

    /// Complete the setup of the PeerConnection for a subscriber
    ///
    /// The subscriber is supposed to send a JSEP SDP answer back to the plugin by the means of this request,
    /// which in this case MUST be associated with a JSEP SDP answer but otherwise requires no arguments.
    pub async fn start(
        &self,
        establishment_protocol: EstablishmentProtocol,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        self.handle
            .send_waiton_ack_with_est(json!({"request": "start"}), establishment_protocol, timeout)
            .await?;
        Ok(())
    }

    pub async fn subscribe(
        &self,
        streams: Vec<VideoRoomSubscriberJoinStream>,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        let mut message = serde_json::to_value(streams)?;
        message["request"] = "subscribe".into();

        self.handle.send_waiton_ack(message, timeout).await?;

        Ok(())
    }

    pub async fn unsubscribe(
        &self,
        streams: Vec<VideoRoomSubscriberUnsubscribeStream>,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        let mut message = serde_json::to_value(streams)?;
        message["request"] = "unsubscribe".into();

        self.handle.send_waiton_ack(message, timeout).await?;

        Ok(())
    }

    #[cfg(feature = "__experimental")]
    pub async fn update(
        &self,
        subscribe: Vec<VideoRoomSubscriberJoinStream>,
        unsubscribe: Vec<VideoRoomSubscriberUnsubscribeStream>,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        let message = json!({
            "request": "update",
            "subscribe": subscribe,
            "unsubscribe": unsubscribe,
        });

        self.handle.send_waiton_ack(message, timeout).await?;

        Ok(())
    }

    #[cfg(feature = "__experimental")]
    pub async fn pause(&self, timeout: Duration) -> Result<(), jarust_interface::Error> {
        self.handle
            .send_waiton_ack(json!({"request": "pause"}), timeout)
            .await?;
        Ok(())
    }

    #[cfg(feature = "__experimental")]
    pub async fn switch(
        &self,
        streams: Vec<VideoRoomSwitchStream>,
        timeout: Duration,
    ) -> Result<(), jarust_interface::Error> {
        let mut message = serde_json::to_value(streams)?;
        message["request"] = "switch".into();

        self.handle.send_waiton_ack(message, timeout).await?;

        Ok(())
    }

    pub async fn leave(&self, timeout: Duration) -> Result<(), jarust_interface::Error> {
        self.handle
            .send_waiton_ack(json!({"request": "leave"}), timeout)
            .await?;
        Ok(())
    }
}

impl PluginTask for VideoRoomHandle {
    fn assign_task(&mut self, task: JaTask) {
        self.task = Some(task);
    }

    fn cancel_task(&mut self) {
        if let Some(task) = self.task.take() {
            task.cancel()
        };
    }
}

impl From<JaHandle> for VideoRoomHandle {
    fn from(handle: JaHandle) -> Self {
        Self { handle, task: None }
    }
}

impl Deref for VideoRoomHandle {
    type Target = JaHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Drop for VideoRoomHandle {
    fn drop(&mut self) {
        self.cancel_task();
    }
}
