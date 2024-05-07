use jarust::error::JaError;
use jarust::japrotocol::EstablishmentProtocol;
use jarust::japrotocol::GenericEvent;
use jarust::japrotocol::JaHandleEvent;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::ResponseType;
use serde::Deserialize;
use serde_json::from_value;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "audiobridge")]
enum AudioBridgeEventDto {
    #[serde(rename = "joined")]
    JoinRoom {
        id: u64,
        room: u64,
        participants: Vec<Participant>,
    },
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Participant {
    pub id: u64,
    pub display: Option<String>,
    pub setup: bool,
    pub muted: bool,
    pub suspended: bool,
    pub talking: bool,
    pub spatial_position: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum PluginEvent {
    AudioBridgeEvent(AudioBridgeEvent),
    GenericEvent(GenericEvent),
}

#[derive(Debug, PartialEq)]
pub enum AudioBridgeEvent {
    JoinRoom {
        id: u64,
        room: u64,
        participants: Vec<Participant>,
        establishment_protocol: EstablishmentProtocol,
    },
}

impl TryFrom<JaResponse> for PluginEvent {
    type Error = JaError;

    fn try_from(value: JaResponse) -> Result<Self, Self::Error> {
        match value.janus {
            ResponseType::Event(JaHandleEvent::PluginEvent { plugin_data }) => {
                match value.establishment_protocol {
                    Some(establishment_protocol) => {
                        match from_value::<AudioBridgeEventDto>(plugin_data.data)? {
                            AudioBridgeEventDto::JoinRoom {
                                id,
                                room,
                                participants,
                            } => Ok(PluginEvent::AudioBridgeEvent(AudioBridgeEvent::JoinRoom {
                                id,
                                room,
                                participants,
                                establishment_protocol,
                            })),
                        }
                    }
                    None => Err(JaError::IncompletePacket),
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
    use jarust::japrotocol::EstablishmentProtocol;
    use jarust::japrotocol::JaHandleEvent;
    use jarust::japrotocol::JaResponse;
    use jarust::japrotocol::Jsep;
    use jarust::japrotocol::JsepType;
    use jarust::japrotocol::PluginData;
    use jarust::japrotocol::ResponseType;
    use serde_json::json;

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
            PluginEvent::AudioBridgeEvent(AudioBridgeEvent::JoinRoom {
                id: 7513785212278430,
                room: 6846571539994870,
                participants: vec![],
                establishment_protocol: EstablishmentProtocol::JSEP(Jsep {
                    jsep_type: JsepType::Answer,
                    sdp: "test_sdp".to_string()
                })
            })
        );
    }
}
