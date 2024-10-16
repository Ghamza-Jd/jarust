use super::common::AudioBridgeParticipant;
use crate::JanusId;
use jarust_interface::japrotocol::EstProto;
use jarust_interface::japrotocol::GenericEvent;
use jarust_interface::japrotocol::JaHandleEvent;
use jarust_interface::japrotocol::JaResponse;
use jarust_interface::japrotocol::PluginInnerData;
use jarust_interface::japrotocol::ResponseType;
use serde::Deserialize;
use serde_json::from_value;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(tag = "audiobridge")]
enum AudioBridgeEventDto {
    #[serde(rename = "joined")]
    RoomJoined {
        id: JanusId,
        room: JanusId,
        participants: Vec<AudioBridgeParticipant>,
    },

    #[serde(rename = "left")]
    RoomLeft { id: JanusId, room: JanusId },

    #[serde(rename = "roomchanged")]
    RoomChanged {
        id: JanusId,
        room: JanusId,
        participants: Vec<AudioBridgeParticipant>,
    },

    #[serde(rename = "event")]
    Event {
        room: u64,
        participants: Vec<AudioBridgeParticipant>,
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
        participants: Vec<AudioBridgeParticipant>,
        estproto: EstProto,
    },
    RoomJoined {
        id: JanusId,
        room: JanusId,
        participants: Vec<AudioBridgeParticipant>,
    },
    RoomLeft {
        id: JanusId,
        room: JanusId,
    },
    RoomChanged {
        id: JanusId,
        room: JanusId,
        participants: Vec<AudioBridgeParticipant>,
    },
    ParticipantsUpdated {
        room: u64,
        participants: Vec<AudioBridgeParticipant>,
    },
    Error {
        error_code: u16,
        error: String,
    },
}

impl TryFrom<JaResponse> for PluginEvent {
    type Error = jarust_interface::Error;

    fn try_from(value: JaResponse) -> Result<Self, Self::Error> {
        match value.janus {
            ResponseType::Event(JaHandleEvent::PluginEvent { plugin_data }) => {
                let audiobridge_event = match plugin_data.data {
                    PluginInnerData::Error { error_code, error } => {
                        AudioBridgeEvent::Error { error_code, error }
                    }
                    PluginInnerData::Data(data) => match from_value::<AudioBridgeEventDto>(data)? {
                        AudioBridgeEventDto::RoomJoined {
                            id,
                            room,
                            participants,
                        } => match value.estproto {
                            Some(estproto) => AudioBridgeEvent::RoomJoinedWithEstabilshment {
                                id,
                                room,
                                participants,
                                estproto,
                            },
                            None => AudioBridgeEvent::RoomJoined {
                                id,
                                room,
                                participants,
                            },
                        },
                        AudioBridgeEventDto::RoomLeft { id, room } => {
                            AudioBridgeEvent::RoomLeft { id, room }
                        }
                        AudioBridgeEventDto::RoomChanged {
                            id,
                            room,
                            participants,
                        } => AudioBridgeEvent::RoomChanged {
                            id,
                            room,
                            participants,
                        },
                        AudioBridgeEventDto::Event { room, participants } => {
                            AudioBridgeEvent::ParticipantsUpdated { room, participants }
                        }
                    },
                };
                Ok(PluginEvent::AudioBridgeEvent(audiobridge_event))
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
    use crate::audio_bridge::events::AudioBridgeEvent;
    use crate::JanusId;
    use jarust_interface::japrotocol::EstProto;
    use jarust_interface::japrotocol::JaHandleEvent;
    use jarust_interface::japrotocol::JaResponse;
    use jarust_interface::japrotocol::Jsep;
    use jarust_interface::japrotocol::JsepType;
    use jarust_interface::japrotocol::PluginData;
    use jarust_interface::japrotocol::PluginInnerData;
    use jarust_interface::japrotocol::ResponseType;
    use serde_json::json;

    #[test]
    fn it_parse_room_joined() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.audiobridge".to_string(),
                    data: PluginInnerData::Data(json!({
                        "audiobridge": "joined",
                        "room": 6846571539994870u64,
                        "id": 7513785212278430u64,
                        "participants": []
                    })),
                },
            }),
            estproto: None,
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
                    data: PluginInnerData::Data(json!({
                        "audiobridge": "joined",
                        "room": 6846571539994870u64,
                        "id": 7513785212278430u64,
                        "participants": []
                    })),
                },
            }),
            estproto: Some(EstProto::JSEP(Jsep {
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
                estproto: EstProto::JSEP(Jsep {
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
                    data: PluginInnerData::Data(json!({
                        "audiobridge": "left",
                        "room": 6846571539994870u64,
                        "id": 7513785212278430u64
                    })),
                },
            }),
            estproto: None,
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
                    data: PluginInnerData::Data(json!({
                        "audiobridge": "roomchanged",
                        "room": 6168266702836626u64,
                        "id": 3862697705388820u64,
                        "participants": []
                    })),
                },
            }),
            estproto: None,
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
