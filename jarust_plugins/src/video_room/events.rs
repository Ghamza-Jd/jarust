use jarust::error::JaError;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::{EstablishmentProtocol, GenericEvent, JaHandleEvent, ResponseType};
use serde::Deserialize;
use serde_json::from_value;

use crate::video_room::responses::{AttachedStream, Attendee, ConfiguredStream, Publisher};
use crate::Identifier;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PluginEvent {
    VideoRoomEvent(VideoRoomEvent),
    GenericEvent(GenericEvent),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(tag = "videoroom")]
enum VideoRoomEventDto {
    #[serde(rename = "destroyed")]
    DestroyRoom { room: Identifier },

    #[serde(rename = "joining")]
    JoinedRoom {
        id: Identifier,
        room: Identifier,
        display: Option<String>,
    },

    #[serde(rename = "publishers")]
    NewPublisher {
        room: Identifier,
        publishers: Vec<Publisher>,
    },

    #[serde(rename = "joined")]
    PublisherJoined {
        room: Identifier,
        description: Option<String>,
        id: Identifier,
        private_id: u64,
        publishers: Vec<Publisher>,
        attendees: Vec<Attendee>,
    },

    #[serde(rename = "attached")]
    SubscriberAttached {
        room: Identifier,
        streams: Vec<AttachedStream>,
    },

    #[serde(rename = "updated")]
    SubscriberUpdated {
        room: Identifier,
        streams: Vec<AttachedStream>,
    },

    #[serde(rename = "talking")]
    Talking {
        room: Identifier,
        id: Identifier,
        #[serde(rename = "audio-level-dBov-avg")]
        audio_level: i16,
    },

    #[serde(rename = "stopped-talking")]
    StoppedTalking {
        room: Identifier,
        id: Identifier,
        #[serde(rename = "audio-level-dBov-avg")]
        audio_level: i16,
    },

    #[serde(rename = "event")]
    Event(VideoRoomEventEventType),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(untagged)]
enum VideoRoomEventEventType {
    #[serde(rename = "error")]
    ErrorEvent { error_code: u16, error: String },

    #[serde(rename = "publishers")]
    PublishersEvent {
        room: Identifier,
        publishers: Vec<Publisher>,
    },

    #[serde(rename = "unpublished")]
    UnpublishedRsp,

    #[serde(rename = "unpublished")]
    UnpublishedEvent {
        room: Identifier,
        unpublished: Identifier,
    },

    #[serde(rename = "configured")]
    ConfiguredRsp {
        room: Identifier,
        audio_codec: Option<String>,
        video_codec: Option<String>,
        streams: Vec<ConfiguredStream>,
    },

    #[serde(rename = "leaving")]
    LeavingEvent {
        room: Identifier,
        leaving: Identifier,
    },

    #[serde(rename = "started")]
    StartedRsp,

    #[serde(rename = "paused")]
    PausedRsp,

    #[serde(rename = "switched")]
    SwitchedRsp {
        room: Identifier,
        changes: i64,
        streams: Vec<AttachedStream>,
    },

    #[serde(rename = "left")]
    LeftRsp,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum VideoRoomEvent {
    /// Sent to all participants in the video room when the room is destroyed
    RoomDestroyed { room: Identifier },

    /// Sent to all participants if a new participant joins
    RoomJoined {
        /// unique ID of the new participant
        id: Identifier,

        /// ID of the room the participant joined into
        room: Identifier,

        /// display name of the new participant
        display: Option<String>,
    },

    /// Sent to all participants if a new participant joins
    RoomJoinedWithEstablishment {
        /// unique ID of the new participant
        id: Identifier,

        /// display name of the new participant
        display: Option<String>,

        establishment_protocol: EstablishmentProtocol,
    },

    /// Sent to all participants if a participant started publishing
    NewPublisher {
        room: Identifier,
        publishers: Vec<Publisher>,
    },

    PublisherJoined {
        room: Identifier,
        description: Option<String>,
        id: Identifier,
        private_id: u64,
        publishers: Vec<Publisher>,
        attendees: Vec<Attendee>,
    },

    LeftRoom {
        room: Identifier,
        participant: Identifier,
    },

    /// Sent back to a subscriber session after a successful [join_as_subscriber](super::handle::VideoRoomHandle::join_as_subscriber) request accompanied by a new JSEP SDP offer
    SubscriberAttached {
        /// unique ID of the room the subscriber joined
        room: Identifier,
        streams: Vec<AttachedStream>,
    },

    SubscriberUpdated {
        room: Identifier,
        streams: Vec<AttachedStream>,
    },

    SubscriberSwitched {
        room: Identifier,
        changes: i64,
        streams: Vec<AttachedStream>,
    },

    /// Sent back to a publisher session after a successful [publish](super::handle::VideoRoomHandle::publish) or
    /// [configure_publisher](super::handle::VideoRoomHandle::configure_publisher) request
    Configured {
        room: Identifier,
        audio_codec: Option<String>,
        video_codec: Option<String>,
        streams: Vec<ConfiguredStream>,
    },

    /// Sent back to a publisher session after a successful [publish](super::handle::VideoRoomHandle::publish) or
    /// [configure_publisher](super::handle::VideoRoomHandle::configure_publisher) request
    ConfiguredWithEstablishment {
        room: Identifier,
        audio_codec: Option<String>,
        video_codec: Option<String>,
        streams: Vec<ConfiguredStream>,
        establishment_protocol: EstablishmentProtocol,
    },

    /// When configuring the room to request the ssrc-audio-level RTP extension,
    /// ad-hoc events might be sent to all publishers if audiolevel_event is set to true
    Talking {
        /// unique ID of the room the publisher is in
        room: Identifier,

        /// unique ID of the publisher
        id: Identifier,

        /// average value of audio level, 127=muted, 0='too loud'
        audio_level: i16,
    },

    /// When configuring the room to request the ssrc-audio-level RTP extension,
    /// ad-hoc events might be sent to all publishers if audiolevel_event is set to true
    StoppedTalking {
        /// unique ID of the room the publisher is in
        room: Identifier,

        /// unique ID of the publisher
        id: Identifier,

        /// average value of audio level, 127=muted, 0='too loud'
        audio_level: i16,
    },

    /// As soon as the PeerConnection is gone, all the other participants will
    /// also be notified about the fact that the stream is no longer available
    Unpublished {
        /// unique ID of the room the publisher is in
        room: Identifier,

        /// unique ID of the publisher
        id: Identifier,
    },

    /// Sent back to a publisher after a successful [unpublish](super::handle::VideoRoomHandle::unpublish) request
    UnpublishedAsyncRsp,

    /// Sent back to a subscriber after a successful [start](super::handle::VideoRoomHandle::start) request
    StartedAsyncRsp,

    /// Sent back to a subscriber after a successful [pause](super::handle::VideoRoomHandle::pause) request
    PausedAsyncRsp,

    /// Sent back to a subscriber after a successful [leave](super::handle::VideoRoomHandle::leave) request
    LeftAsyncRsp,
}

impl TryFrom<JaResponse> for PluginEvent {
    type Error = JaError;

    fn try_from(value: JaResponse) -> Result<Self, Self::Error> {
        match value.janus {
            ResponseType::Event(JaHandleEvent::PluginEvent { plugin_data }) => {
                let videoroom_event = from_value::<VideoRoomEventDto>(plugin_data.data)?;
                match videoroom_event {
                    VideoRoomEventDto::DestroyRoom { room } => {
                        Ok(PluginEvent::VideoRoomEvent(VideoRoomEvent::RoomDestroyed {
                            room,
                        }))
                    }

                    VideoRoomEventDto::JoinedRoom { id, room, display } => {
                        match value.establishment_protocol {
                            Some(establishment_protocol) => Ok(PluginEvent::VideoRoomEvent(
                                VideoRoomEvent::RoomJoinedWithEstablishment {
                                    id,
                                    display,
                                    establishment_protocol,
                                },
                            )),
                            None => Ok(PluginEvent::VideoRoomEvent(VideoRoomEvent::RoomJoined {
                                id,
                                room,
                                display,
                            })),
                        }
                    }

                    VideoRoomEventDto::NewPublisher { room, publishers } => {
                        Ok(PluginEvent::VideoRoomEvent(VideoRoomEvent::NewPublisher {
                            room,
                            publishers,
                        }))
                    }

                    VideoRoomEventDto::PublisherJoined {
                        room,
                        description,
                        id,
                        private_id,
                        publishers,
                        attendees,
                    } => Ok(PluginEvent::VideoRoomEvent(
                        VideoRoomEvent::PublisherJoined {
                            room,
                            description,
                            id,
                            private_id,
                            publishers,
                            attendees,
                        },
                    )),

                    VideoRoomEventDto::SubscriberAttached { room, streams } => {
                        Ok(PluginEvent::VideoRoomEvent(
                            VideoRoomEvent::SubscriberAttached { room, streams },
                        ))
                    }

                    VideoRoomEventDto::SubscriberUpdated { room, streams } => {
                        Ok(PluginEvent::VideoRoomEvent(
                            VideoRoomEvent::SubscriberUpdated { room, streams },
                        ))
                    }

                    VideoRoomEventDto::Talking {
                        room,
                        id,
                        audio_level,
                    } => Ok(PluginEvent::VideoRoomEvent(VideoRoomEvent::Talking {
                        room,
                        id,
                        audio_level,
                    })),

                    VideoRoomEventDto::StoppedTalking {
                        room,
                        id,
                        audio_level,
                    } => Ok(PluginEvent::VideoRoomEvent(
                        VideoRoomEvent::StoppedTalking {
                            room,
                            id,
                            audio_level,
                        },
                    )),

                    VideoRoomEventDto::Event(e) => match e {
                        VideoRoomEventEventType::ErrorEvent { error_code, error } => {
                            Err(JaError::JanusError {
                                code: error_code,
                                reason: error,
                            })
                        }
                        VideoRoomEventEventType::PublishersEvent { room, publishers } => {
                            Ok(PluginEvent::VideoRoomEvent(VideoRoomEvent::NewPublisher {
                                room,
                                publishers,
                            }))
                        }
                        VideoRoomEventEventType::UnpublishedRsp {} => Ok(
                            PluginEvent::VideoRoomEvent(VideoRoomEvent::UnpublishedAsyncRsp),
                        ),
                        VideoRoomEventEventType::UnpublishedEvent { room, unpublished } => {
                            Ok(PluginEvent::VideoRoomEvent(VideoRoomEvent::Unpublished {
                                room,
                                id: unpublished,
                            }))
                        }
                        VideoRoomEventEventType::ConfiguredRsp {
                            room,
                            audio_codec,
                            video_codec,
                            streams,
                        } => {
                            if let Some(establishment_protocol) = value.establishment_protocol {
                                Ok(PluginEvent::VideoRoomEvent(
                                    VideoRoomEvent::ConfiguredWithEstablishment {
                                        room,
                                        audio_codec,
                                        video_codec,
                                        streams,
                                        establishment_protocol,
                                    },
                                ))
                            } else {
                                Ok(PluginEvent::VideoRoomEvent(VideoRoomEvent::Configured {
                                    room,
                                    audio_codec,
                                    video_codec,
                                    streams,
                                }))
                            }
                        }
                        VideoRoomEventEventType::LeavingEvent { room, leaving } => {
                            Ok(PluginEvent::VideoRoomEvent(VideoRoomEvent::LeftRoom {
                                room,
                                participant: leaving,
                            }))
                        }
                        VideoRoomEventEventType::StartedRsp => {
                            Ok(PluginEvent::VideoRoomEvent(VideoRoomEvent::StartedAsyncRsp))
                        }
                        VideoRoomEventEventType::PausedRsp => {
                            Ok(PluginEvent::VideoRoomEvent(VideoRoomEvent::PausedAsyncRsp))
                        }
                        VideoRoomEventEventType::SwitchedRsp {
                            room,
                            changes,
                            streams,
                        } => Ok(PluginEvent::VideoRoomEvent(
                            VideoRoomEvent::SubscriberSwitched {
                                room,
                                changes,
                                streams,
                            },
                        )),
                        VideoRoomEventEventType::LeftRsp => {
                            Ok(PluginEvent::VideoRoomEvent(VideoRoomEvent::LeftAsyncRsp))
                        }
                    },
                }
            }
            ResponseType::Event(JaHandleEvent::GenericEvent(event)) => {
                Ok(PluginEvent::GenericEvent(event))
            }
            _ => Err(JaError::IncompletePacket),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use jarust::error::JaError;
    use jarust::japrotocol::{
        EstablishmentProtocol, JaHandleEvent, JaResponse, Jsep, JsepType, PluginData, ResponseType,
    };

    use super::PluginEvent;
    use crate::video_room::events::VideoRoomEvent;
    use crate::video_room::responses::ConfiguredStream;
    use crate::Identifier;

    #[test]
    fn it_parse_joined_room() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: json!({
                        "videoroom": "joining",
                        "room": 8812066423493633u64,
                        "id": 6380744183070564u64,
                        "display": "Joiner McJoinface"
                    }),
                },
            }),
            establishment_protocol: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::RoomJoined {
                id: Identifier::Uint(6380744183070564),
                room: Identifier::Uint(8812066423493633u64),
                display: Some("Joiner McJoinface".to_string()),
            })
        );
    }

    #[test]
    fn it_parse_joined_room_with_establishment() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: json!({
                        "videoroom": "joining",
                        "room": 8812066423493633u64,
                        "id": 6380744183070564u64,
                        "display": "Joiner McJoinface"
                    }),
                },
            }),
            establishment_protocol: Some(EstablishmentProtocol::JSEP(Jsep {
                jsep_type: JsepType::Answer,
                sdp: "test_sdp".to_string(),
            })),
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::RoomJoinedWithEstablishment {
                id: Identifier::Uint(6380744183070564u64),
                display: Some("Joiner McJoinface".to_string()),
                establishment_protocol: EstablishmentProtocol::JSEP(Jsep {
                    jsep_type: JsepType::Answer,
                    sdp: "test_sdp".to_string(),
                })
            })
        )
    }

    #[test]
    fn it_parse_destroy_room() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: json!({
                        "videoroom": "destroyed",
                        "room": 8812066423493633u64,
                    }),
                },
            }),
            establishment_protocol: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::RoomDestroyed {
                room: Identifier::Uint(8812066423493633u64),
            })
        )
    }

    #[test]
    fn it_parse_new_publisher() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: json!({
                        "videoroom": "publishers",
                        "room": 8812066423493633u64,
                        "publishers": []
                    }),
                },
            }),
            establishment_protocol: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::NewPublisher {
                room: Identifier::Uint(8812066423493633u64),
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
                    data: json!({
                       "videoroom": "joined",
                       "room": 3966653182028680u64,
                       "description": "A brand new description!",
                       "id": 1337,
                       "private_id": 4113762326u64,
                       "publishers": [],
                       "attendees": []
                    }),
                },
            }),
            establishment_protocol: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::PublisherJoined {
                room: Identifier::Uint(3966653182028680),
                description: Some("A brand new description!".to_string()),
                id: Identifier::Uint(1337),
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
                    data: json!({
                        "videoroom": "event",
                        "error_code": 429,
                        "error": "Missing mandatory element (feed)"
                    }),
                },
            }),
            establishment_protocol: None,
            transaction: None,
            session_id: None,
            sender: None,
        };
        let result: Result<PluginEvent, JaError> = rsp.try_into();
        assert!(result.is_err());
        //assert_eq!(result.err().is_some(), true);
        let ja_error = result.err();
        assert!(ja_error.is_some());
        assert_eq!(
            ja_error.unwrap().to_string(),
            "Janus error { code: 429, reason: Missing mandatory element (feed)}"
        );
    }

    #[test]
    fn it_parse_leaving() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: json!({
                       "videoroom": "event",
                       "room": 8146468481724441u64,
                       "leaving": "ok"
                    }),
                },
            }),
            establishment_protocol: None,
            sender: None,
            session_id: None,
            transaction: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::LeftRoom {
                room: Identifier::Uint(8146468481724441u64),
                participant: Identifier::String("ok".to_string())
            })
        )
    }

    #[test]
    fn it_parse_configured_with_jsep() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.videoroom".to_string(),
                    data: json!({
                       "videoroom": "event",
                       "room": 1657434645789453u64,
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
                    }),
                },
            }),
            transaction: None,
            session_id: None,
            sender: None,
            establishment_protocol: Some(EstablishmentProtocol::JSEP(Jsep {
                jsep_type: JsepType::Answer,
                sdp: "test_sdp".to_string(),
            })),
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::VideoRoomEvent(VideoRoomEvent::ConfiguredWithEstablishment {
                room: Identifier::Uint(1657434645789453u64),
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
                establishment_protocol: EstablishmentProtocol::JSEP(Jsep {
                    jsep_type: JsepType::Answer,
                    sdp: "test_sdp".to_string(),
                })
            })
        )
    }
}
