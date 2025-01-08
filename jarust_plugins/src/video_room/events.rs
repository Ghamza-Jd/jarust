use crate::video_room::responses::AttachedStream;
use crate::video_room::responses::Attendee;
use crate::video_room::responses::ConfiguredStream;
use crate::video_room::responses::Publisher;
use crate::JanusId;
use jarust_core::prelude::JaResponse;
use jarust_interface::japrotocol::GenericEvent;
use jarust_interface::japrotocol::JaHandleEvent;
use jarust_interface::japrotocol::Jsep;
use jarust_interface::japrotocol::PluginInnerData;
use jarust_interface::japrotocol::ResponseType;
use serde::Deserialize;
use serde_json::from_value;
use serde_json::Value;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(tag = "videoroom")]
enum VideoRoomEventDto {
    #[serde(rename = "destroyed")]
    DestroyRoom { room: JanusId },

    #[serde(rename = "joining")]
    JoinedRoom {
        id: JanusId,
        room: JanusId,
        display: Option<String>,
    },

    #[serde(rename = "publishers")]
    NewPublisher {
        room: JanusId,
        publishers: Vec<Publisher>,
    },

    #[serde(rename = "joined")]
    PublisherJoined {
        room: JanusId,
        description: Option<String>,
        id: JanusId,
        private_id: u64,
        publishers: Vec<Publisher>,
        attendees: Vec<Attendee>,
    },

    #[serde(rename = "attached")]
    SubscriberAttached {
        room: JanusId,
        streams: Vec<AttachedStream>,
    },

    #[serde(rename = "updated")]
    SubscriberUpdated {
        room: JanusId,
        streams: Vec<AttachedStream>,
    },

    #[serde(rename = "talking")]
    Talking {
        room: JanusId,
        id: JanusId,
        #[serde(rename = "audio-level-dBov-avg")]
        audio_level: i16,
    },

    #[serde(rename = "stopped-talking")]
    StoppedTalking {
        room: JanusId,
        id: JanusId,
        #[serde(rename = "audio-level-dBov-avg")]
        audio_level: i16,
    },

    #[serde(rename = "event")]
    Event(VideoRoomEventEventType),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(untagged)]
enum VideoRoomEventEventType {
    #[serde(rename = "publishers")]
    PublishersEvent {
        room: JanusId,
        publishers: Vec<Publisher>,
    },

    #[serde(rename = "unpublished")]
    UnpublishedRsp,

    #[serde(rename = "unpublished")]
    UnpublishedEvent { room: JanusId, unpublished: JanusId },

    #[serde(rename = "configured")]
    ConfiguredRsp {
        room: JanusId,
        audio_codec: Option<String>,
        video_codec: Option<String>,
        streams: Vec<ConfiguredStream>,
    },

    #[serde(rename = "leaving")]
    LeavingEvent { room: JanusId, leaving: JanusId },

    #[serde(rename = "started")]
    StartedRsp,

    #[serde(rename = "paused")]
    PausedRsp,

    #[serde(rename = "switched")]
    SwitchedRsp {
        room: JanusId,
        changes: i64,
        streams: Vec<AttachedStream>,
    },

    #[serde(rename = "left")]
    LeftRsp,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum PluginEvent {
    VideoRoomEvent(VideoRoomEvent),
    GenericEvent(GenericEvent),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum VideoRoomEvent {
    /// Sent to all participants in the video room when the room is destroyed
    RoomDestroyed {
        room: JanusId,
    },

    /// Sent to all participants if a new participant joins
    RoomJoined {
        /// unique ID of the new participant
        id: JanusId,

        /// ID of the room the participant joined into
        room: JanusId,

        /// display name of the new participant
        display: Option<String>,
    },

    /// Sent to all participants if a new participant joins
    RoomJoinedWithEst {
        /// unique ID of the new participant
        id: JanusId,

        /// display name of the new participant
        display: Option<String>,

        jsep: Jsep,
    },

    /// Sent to all participants if a participant started publishing
    NewPublisher {
        room: JanusId,
        publishers: Vec<Publisher>,
    },

    PublisherJoined {
        room: JanusId,
        description: Option<String>,
        id: JanusId,
        private_id: u64,
        publishers: Vec<Publisher>,
        attendees: Vec<Attendee>,
    },

    LeftRoom {
        room: JanusId,
        participant: JanusId,
    },

    /// Sent back to a subscriber session after a successful [join_as_subscriber](super::handle::VideoRoomHandle::join_as_subscriber) request accompanied by a new JSEP SDP offer
    SubscriberAttached {
        /// unique ID of the room the subscriber joined
        room: JanusId,
        streams: Vec<AttachedStream>,
    },

    SubscriberUpdated {
        room: JanusId,
        streams: Vec<AttachedStream>,
    },

    SubscriberSwitched {
        room: JanusId,
        changes: i64,
        streams: Vec<AttachedStream>,
    },

    /// Sent back to a publisher session after a successful [publish](super::handle::VideoRoomHandle::publish) or
    /// [configure_publisher](super::handle::VideoRoomHandle::configure_publisher) request
    Configured {
        room: JanusId,
        audio_codec: Option<String>,
        video_codec: Option<String>,
        streams: Vec<ConfiguredStream>,
    },

    /// Sent back to a publisher session after a successful [publish](super::handle::VideoRoomHandle::publish) or
    /// [configure_publisher](super::handle::VideoRoomHandle::configure_publisher) request
    ConfiguredWithEst {
        room: JanusId,
        audio_codec: Option<String>,
        video_codec: Option<String>,
        streams: Vec<ConfiguredStream>,
        jsep: Jsep,
    },

    /// When configuring the room to request the ssrc-audio-level RTP extension,
    /// ad-hoc events might be sent to all publishers if audiolevel_event is set to true
    Talking {
        /// unique ID of the room the publisher is in
        room: JanusId,

        /// unique ID of the publisher
        id: JanusId,

        /// average value of audio level, 127=muted, 0='too loud'
        audio_level: i16,
    },

    /// When configuring the room to request the ssrc-audio-level RTP extension,
    /// ad-hoc events might be sent to all publishers if audiolevel_event is set to true
    StoppedTalking {
        /// unique ID of the room the publisher is in
        room: JanusId,

        /// unique ID of the publisher
        id: JanusId,

        /// average value of audio level, 127=muted, 0='too loud'
        audio_level: i16,
    },

    /// As soon as the PeerConnection is gone, all the other participants will
    /// also be notified about the fact that the stream is no longer available
    Unpublished {
        /// unique ID of the room the publisher is in
        room: JanusId,

        /// unique ID of the publisher
        id: JanusId,
    },

    /// Sent back to a publisher after a successful [unpublish](super::handle::VideoRoomHandle::unpublish) request
    UnpublishedAsyncRsp,

    /// Sent back to a subscriber after a successful [start](super::handle::VideoRoomHandle::start) request
    StartedAsyncRsp,

    /// Sent back to a subscriber after a successful [pause](super::handle::VideoRoomHandle::pause) request
    PausedAsyncRsp,

    /// Sent back to a subscriber after a successful [leave](super::handle::VideoRoomHandle::leave) request
    LeftAsyncRsp,

    Error {
        error_code: u16,
        error: String,
    },

    Other(Value),
}

impl TryFrom<JaResponse> for PluginEvent {
    type Error = jarust_interface::Error;

    fn try_from(value: JaResponse) -> Result<Self, Self::Error> {
        use VideoRoomEventDto as EventDto;
        use VideoRoomEventEventType as Event;

        match value.janus {
            ResponseType::Event(JaHandleEvent::PluginEvent { plugin_data }) => {
                let videoroom_event = match plugin_data.data {
                    PluginInnerData::Error { error_code, error } => {
                        VideoRoomEvent::Error { error_code, error }
                    }
                    PluginInnerData::Data(data) => match from_value::<EventDto>(data.clone()) {
                        Ok(event) => match event {
                            EventDto::DestroyRoom { room } => {
                                VideoRoomEvent::RoomDestroyed { room }
                            }
                            EventDto::JoinedRoom { id, room, display } => match value.jsep {
                                Some(jsep) => {
                                    VideoRoomEvent::RoomJoinedWithEst { id, display, jsep }
                                }

                                None => VideoRoomEvent::RoomJoined { id, room, display },
                            },
                            EventDto::NewPublisher { room, publishers } => {
                                VideoRoomEvent::NewPublisher { room, publishers }
                            }
                            EventDto::PublisherJoined {
                                room,
                                description,
                                id,
                                private_id,
                                publishers,
                                attendees,
                            } => VideoRoomEvent::PublisherJoined {
                                room,
                                description,
                                id,
                                private_id,
                                publishers,
                                attendees,
                            },
                            EventDto::SubscriberAttached { room, streams } => {
                                VideoRoomEvent::SubscriberAttached { room, streams }
                            }
                            EventDto::SubscriberUpdated { room, streams } => {
                                VideoRoomEvent::SubscriberUpdated { room, streams }
                            }
                            EventDto::Talking {
                                room,
                                id,
                                audio_level,
                            } => VideoRoomEvent::Talking {
                                room,
                                id,
                                audio_level,
                            },
                            EventDto::StoppedTalking {
                                room,
                                id,
                                audio_level,
                            } => VideoRoomEvent::StoppedTalking {
                                room,
                                id,
                                audio_level,
                            },
                            EventDto::Event(Event::PublishersEvent { room, publishers }) => {
                                VideoRoomEvent::NewPublisher { room, publishers }
                            }
                            EventDto::Event(Event::UnpublishedRsp {}) => {
                                VideoRoomEvent::UnpublishedAsyncRsp
                            }
                            EventDto::Event(Event::UnpublishedEvent { room, unpublished }) => {
                                VideoRoomEvent::Unpublished {
                                    room,
                                    id: unpublished,
                                }
                            }
                            EventDto::Event(Event::ConfiguredRsp {
                                room,
                                audio_codec,
                                video_codec,
                                streams,
                            }) => {
                                if let Some(jsep) = value.jsep {
                                    VideoRoomEvent::ConfiguredWithEst {
                                        room,
                                        audio_codec,
                                        video_codec,
                                        streams,
                                        jsep,
                                    }
                                } else {
                                    VideoRoomEvent::Configured {
                                        room,
                                        audio_codec,
                                        video_codec,
                                        streams,
                                    }
                                }
                            }
                            EventDto::Event(Event::LeavingEvent { room, leaving }) => {
                                VideoRoomEvent::LeftRoom {
                                    room,
                                    participant: leaving,
                                }
                            }
                            EventDto::Event(Event::StartedRsp) => VideoRoomEvent::StartedAsyncRsp,
                            EventDto::Event(Event::PausedRsp) => VideoRoomEvent::PausedAsyncRsp,
                            EventDto::Event(Event::SwitchedRsp {
                                room,
                                changes,
                                streams,
                            }) => VideoRoomEvent::SubscriberSwitched {
                                room,
                                changes,
                                streams,
                            },
                            EventDto::Event(Event::LeftRsp) => VideoRoomEvent::LeftAsyncRsp,
                        },
                        Err(_) => VideoRoomEvent::Other(data),
                    },
                };
                Ok(PluginEvent::VideoRoomEvent(videoroom_event))
            }
            ResponseType::Event(JaHandleEvent::GenericEvent(event)) => {
                Ok(PluginEvent::GenericEvent(event))
            }
            _ => Err(Self::Error::IncompletePacket),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PluginEvent;
    use crate::video_room::events::VideoRoomEvent;
    use crate::video_room::responses::ConfiguredStream;
    use crate::JanusId;
    use jarust_interface::japrotocol::JaHandleEvent;
    use jarust_interface::japrotocol::JaResponse;
    use jarust_interface::japrotocol::Jsep;
    use jarust_interface::japrotocol::JsepType;
    use jarust_interface::japrotocol::PluginData;
    use jarust_interface::japrotocol::PluginInnerData;
    use jarust_interface::japrotocol::ResponseType;
    use serde_json::json;

    #[test]
    fn it_parse_joined_room() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: PluginInnerData::Data(json!({
                        "videoroom": "joining",
                        "room": 88120664u64,
                        "id": 638074u64,
                        "display": "Joiner McJoinface"
                    })),
                },
            }),
            jsep: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::RoomJoined {
                id: JanusId::Uint(638074.into()),
                room: JanusId::Uint(88120664.into()),
                display: Some("Joiner McJoinface".to_string()),
            })
        );
    }

    #[test]
    fn it_parse_joined_room_with_jsep() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: PluginInnerData::Data(json!({
                        "videoroom": "joining",
                        "room": 8812063u64,
                        "id": 8146468u64,
                        "display": "Joiner McJoinface"
                    })),
                },
            }),
            jsep: Some(Jsep {
                jsep_type: JsepType::Answer,
                trickle: Some(false),
                sdp: "test_sdp".to_string(),
            }),
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::RoomJoinedWithEst {
                id: JanusId::Uint(8146468.into()),
                display: Some("Joiner McJoinface".to_string()),
                jsep: Jsep {
                    jsep_type: JsepType::Answer,
                    trickle: Some(false),
                    sdp: "test_sdp".to_string(),
                }
            })
        )
    }

    #[test]
    fn it_parse_destroy_room() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: PluginInnerData::Data(json!({
                        "videoroom": "destroyed",
                        "room": 8146468u64,
                    })),
                },
            }),
            jsep: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::RoomDestroyed {
                room: JanusId::Uint(8146468.into()),
            })
        )
    }

    #[test]
    fn it_parse_new_publisher() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: PluginInnerData::Data(json!({
                        "videoroom": "publishers",
                        "room": 8146468u64,
                        "publishers": []
                    })),
                },
            }),
            jsep: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::NewPublisher {
                room: JanusId::Uint(8146468.into()),
                publishers: vec![]
            })
        );
    }

    #[test]
    fn it_parse_publisher_joined() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: PluginInnerData::Data(json!({
                       "videoroom": "joined",
                       "room": 8146468u64,
                       "description": "A brand new description!",
                       "id": 1337,
                       "private_id": 4113762326u64,
                       "publishers": [],
                       "attendees": []
                    })),
                },
            }),
            jsep: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::PublisherJoined {
                room: JanusId::Uint(8146468.into()),
                description: Some("A brand new description!".to_string()),
                id: JanusId::Uint(1337.into()),
                private_id: 4113762326,
                publishers: vec![],
                attendees: vec![]
            })
        )
    }

    #[test]
    fn it_parse_error() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: PluginInnerData::Error {
                        error_code: 429,
                        error: "Missing mandatory element (feed)".to_string(),
                    },
                },
            }),
            jsep: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::Error {
                error_code: 429,
                error: "Missing mandatory element (feed)".to_string()
            })
        );
    }

    #[test]
    fn it_parse_leaving() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: PluginInnerData::Data(json!({
                       "videoroom": "event",
                       "room": 8146468u64,
                       "leaving": "ok"
                    })),
                },
            }),
            jsep: None,
            sender: None,
            session_id: None,
            transaction: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::LeftRoom {
                room: JanusId::Uint(8146468.into()),
                participant: JanusId::String("ok".to_string())
            })
        )
    }

    #[test]
    fn it_parse_configured_with_jsep() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: PluginInnerData::Data(json!({
                       "videoroom": "event",
                       "room": 8146468u64,
                       "configured": "ok",
                       "audio_codec": "opus",
                       "video_codec": "h264",
                       "streams": [
                          {
                            "type": "audio",
                            "mindex": 0,
                            "mid": "0",
                            "codec": "opus",
                            "stereo": true,
                            "fec": true
                          },
                          {
                            "type": "video",
                            "mindex": 1,
                            "mid": "1",
                            "codec": "h264",
                            "h264_profile": "42e01f"
                          }
                       ]
                    })),
                },
            }),
            transaction: None,
            session_id: None,
            sender: None,
            jsep: Some(Jsep {
                jsep_type: JsepType::Answer,
                trickle: Some(false),
                sdp: "test_sdp".to_string(),
            }),
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::ConfiguredWithEst {
                room: JanusId::Uint(8146468.into()),
                audio_codec: Some("opus".to_string()),
                video_codec: Some("h264".to_string()),
                streams: vec![
                    ConfiguredStream {
                        media_type: "audio".to_string(),
                        mindex: 0,
                        mid: "0".to_string(),
                        codec: "opus".to_string(),
                        stereo: true,
                        fec: true,
                        ..Default::default()
                    },
                    ConfiguredStream {
                        media_type: "video".to_string(),
                        mindex: 1,
                        mid: "1".to_string(),
                        codec: "h264".to_string(),
                        h264_profile: Some("42e01f".to_string()),
                        ..Default::default()
                    }
                ],
                jsep: Jsep {
                    jsep_type: JsepType::Answer,
                    trickle: Some(false),
                    sdp: "test_sdp".to_string(),
                }
            })
        )
    }

    #[test]
    fn it_parse_unsupported_event_as_other() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: PluginInnerData::Data(json!({
                        "videoroom": "jarust_rocks",
                        "room": 6613848040355181645u64,
                        "jarust": "rocks"
                    })),
                },
            }),
            jsep: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::Other(json!({
                "videoroom": "jarust_rocks",
                "room": 6613848040355181645u64,
                "jarust": "rocks"
            })))
        );
    }
}
