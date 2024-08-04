use super::common::Participant;
use crate::Identifier;
use jarust::error::JaError;
use jarust::nw::japrotocol::EstablishmentProtocol;
use jarust::nw::japrotocol::GenericEvent;
use jarust::nw::japrotocol::JaHandleEvent;
use jarust::nw::japrotocol::JaResponse;
use jarust::nw::japrotocol::ResponseType;
use serde::Deserialize;
use serde_json::from_value;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(tag = "audiobridge")]
enum AudioBridgeEventDto {
    #[serde(rename = "joined")]
    RoomJoined {
        id: Identifier,
        room: Identifier,
        participants: Vec<Participant>,
    },

    #[serde(rename = "left")]
    RoomLeft { id: Identifier, room: Identifier },

    #[serde(rename = "roomchanged")]
    RoomChanged {
        id: Identifier,
        room: Identifier,
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
        id: Identifier,
        room: Identifier,
        participants: Vec<Participant>,
        establishment_protocol: EstablishmentProtocol,
    },
    RoomJoined {
        id: Identifier,
        room: Identifier,
        participants: Vec<Participant>,
    },
    RoomLeft {
        id: Identifier,
        room: Identifier,
    },
    RoomChanged {
        id: Identifier,
        room: Identifier,
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
    use crate::Identifier;
    use jarust::nw::japrotocol::EstablishmentProtocol;
    use jarust::nw::japrotocol::JaHandleEvent;
    use jarust::nw::japrotocol::JaResponse;
    use jarust::nw::japrotocol::Jsep;
    use jarust::nw::japrotocol::JsepType;
    use jarust::nw::japrotocol::PluginData;
    use jarust::nw::japrotocol::ResponseType;
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
                id: Identifier::Uint(7513785212278430),
                room: Identifier::Uint(6846571539994870),
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
                id: Identifier::Uint(7513785212278430),
                room: Identifier::Uint(6846571539994870),
                participants: vec![],
                establishment_protocol: EstablishmentProtocol::JSEP(Jsep {
                    jsep_type: JsepType::Answer,
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
                id: Identifier::Uint(7513785212278430),
                room: Identifier::Uint(6846571539994870),
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
                id: Identifier::Uint(3862697705388820),
                room: Identifier::Uint(6168266702836626),
                participants: vec![],
            })
        );
    }
}
