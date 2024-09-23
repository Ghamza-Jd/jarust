use super::common::Participant;
use crate::JanusId;
use jarust::error::JaError;
use jarust_transport::japrotocol::EstablishmentProtocol;
use jarust_transport::japrotocol::GenericEvent;
use jarust_transport::japrotocol::JaHandleEvent;
use jarust_transport::japrotocol::JaResponse;
use jarust_transport::japrotocol::ResponseType;
use serde::Deserialize;
use serde_json::from_value;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(tag = "audiobridge")]
enum AudioBridgeEventDto {
    #[serde(rename = "joined")]
    RoomJoined {
        id: JanusId,
        room: JanusId,
        participants: Vec<Participant>,
    },

    #[serde(rename = "left")]
    RoomLeft { id: JanusId, room: JanusId },

    #[serde(rename = "roomchanged")]
    RoomChanged {
        id: JanusId,
        room: JanusId,
        participants: Vec<Participant>,
    },

    #[serde(rename = "event")]
    Event {
        room: u64,
        participants: Vec<Participant>,
    },
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PluginEvent {
    AudioBridgeEvent(AudioBridgeEvent),
    GenericEvent(GenericEvent),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AudioBridgeEvent {
    RoomJoinedWithEstabilshment {
        id: JanusId,
        room: JanusId,
        participants: Vec<Participant>,
        establishment_protocol: EstablishmentProtocol,
    },
    RoomJoined {
        id: JanusId,
        room: JanusId,
        participants: Vec<Participant>,
    },
    RoomLeft {
        id: JanusId,
        room: JanusId,
    },
    RoomChanged {
        id: JanusId,
        room: JanusId,
        participants: Vec<Participant>,
    },
    ParticipantsUpdated {
        room: u64,
        participants: Vec<Participant>,
    },
}

impl TryFrom<JaResponse> for PluginEvent {
    type Error = JaError;

    fn try_from(value: JaResponse) -> Result<Self, Self::Error> {
        match value.janus {
            ResponseType::Event(JaHandleEvent::PluginEvent { plugin_data }) => {
                let audiobridge_event = from_value::<AudioBridgeEventDto>(plugin_data.data)?;
                match audiobridge_event {
                    AudioBridgeEventDto::RoomJoined {
                        id,
                        room,
                        participants,
                    } => match value.establishment_protocol {
                        Some(establishment_protocol) => Ok(PluginEvent::AudioBridgeEvent(
                            AudioBridgeEvent::RoomJoinedWithEstabilshment {
                                id,
                                room,
                                participants,
                                establishment_protocol,
                            },
                        )),
                        None => Ok(PluginEvent::AudioBridgeEvent(
                            AudioBridgeEvent::RoomJoined {
                                id,
                                room,
                                participants,
                            },
                        )),
                    },

                    AudioBridgeEventDto::RoomLeft { id, room } => {
                        Ok(PluginEvent::AudioBridgeEvent(AudioBridgeEvent::RoomLeft {
                            id,
                            room,
                        }))
                    }

                    AudioBridgeEventDto::RoomChanged {
                        id,
                        room,
                        participants,
                    } => Ok(PluginEvent::AudioBridgeEvent(
                        AudioBridgeEvent::RoomChanged {
                            id,
                            room,
                            participants,
                        },
                    )),

                    AudioBridgeEventDto::Event { room, participants } => {
                        Ok(PluginEvent::AudioBridgeEvent(
                            AudioBridgeEvent::ParticipantsUpdated { room, participants },
                        ))
                    }
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
    use super::PluginEvent;
    use crate::audio_bridge::events::AudioBridgeEvent;
    use crate::JanusId;
    use jarust_transport::japrotocol::EstablishmentProtocol;
    use jarust_transport::japrotocol::JaHandleEvent;
    use jarust_transport::japrotocol::JaResponse;
    use jarust_transport::japrotocol::Jsep;
    use jarust_transport::japrotocol::JsepType;
    use jarust_transport::japrotocol::PluginData;
    use jarust_transport::japrotocol::ResponseType;
    use serde_json::json;

    #[test]
    fn it_parse_room_joined() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.audiobridge".to_string(),
                    data: json!({
                        "audiobridge": "joined",
                        "room": 6846571539994870u64,
                        "id": 7513785212278430u64,
                        "participants": []
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
            PluginEvent::AudioBridgeEvent(AudioBridgeEvent::RoomJoined {
                id: JanusId::Uint(7513785212278430),
                room: JanusId::Uint(6846571539994870),
                participants: vec![],
            })
        );
    }

    #[test]
    fn it_parse_room_joined_with_establishment_event() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.audiobridge".to_string(),
                    data: json!({
                        "audiobridge": "joined",
                        "room": 6846571539994870u64,
                        "id": 7513785212278430u64,
                        "participants": []
                    }),
                },
            }),
            establishment_protocol: Some(EstablishmentProtocol::JSEP(Jsep {
                jsep_type: JsepType::Answer,
                trickle: Some(false),
                sdp: "test_sdp".to_string(),
            })),
            transaction: None,
            session_id: None,
            sender: None,
        };
        let event: PluginEvent = rsp.try_into().unwrap();
        assert_eq!(
            event,
            PluginEvent::AudioBridgeEvent(AudioBridgeEvent::RoomJoinedWithEstabilshment {
                id: JanusId::Uint(7513785212278430),
                room: JanusId::Uint(6846571539994870),
                participants: vec![],
                establishment_protocol: EstablishmentProtocol::JSEP(Jsep {
                    jsep_type: JsepType::Answer,
                    trickle: Some(false),
                    sdp: "test_sdp".to_string(),
                }),
            })
        );
    }

    #[test]
    fn it_parse_room_left() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.audiobridge".to_string(),
                    data: json!({
                        "audiobridge": "left",
                        "room": 6846571539994870u64,
                        "id": 7513785212278430u64
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
            PluginEvent::AudioBridgeEvent(AudioBridgeEvent::RoomLeft {
                id: JanusId::Uint(7513785212278430),
                room: JanusId::Uint(6846571539994870),
            })
        );
    }

    #[test]
    fn it_parse_room_changed() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.audiobridge".to_string(),
                    data: json!({
                        "audiobridge": "roomchanged",
                        "room": 6168266702836626u64,
                        "id": 3862697705388820u64,
                        "participants": []
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
            PluginEvent::AudioBridgeEvent(AudioBridgeEvent::RoomChanged {
                id: JanusId::Uint(3862697705388820),
                room: JanusId::Uint(6168266702836626),
                participants: vec![],
            })
        );
    }
}
