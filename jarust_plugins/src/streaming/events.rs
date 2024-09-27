use crate::JanusId;
use jarust::error::JaError;
use jarust::prelude::JaResponse;
use jarust_transport::error::JaTransportError;
use jarust_transport::japrotocol::GenericEvent;
use jarust_transport::japrotocol::JaHandleEvent;
use jarust_transport::japrotocol::ResponseType;
use serde::Deserialize;
use serde_json::from_value;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PluginEvent {
    StreamingEvent(StreamingEvent),
    GenericEvent(GenericEvent),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(tag = "streaming")]
enum StreamingEventDto {
    #[serde(rename = "destroyed")]
    DestroyMountpoint { id: JanusId },

    #[serde(rename = "created")]
    CreateMountpoint {
        id: JanusId,
        /// <live|on demand>
        #[serde(rename = "type")]
        mountpoint_type: String,
    },

    #[serde(rename = "event")]
    Event(StreamingEventEventType),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(untagged)]
enum StreamingEventEventType {
    #[serde(rename = "error")]
    ErrorEvent { error_code: u16, error: String },
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum StreamingEvent {
    MountpointDestroyed {
        id: JanusId,
    },
    MountpointCreated {
        id: JanusId,
        mountpoint_type: String,
    },
}

impl TryFrom<JaResponse> for PluginEvent {
    type Error = JaError;

    fn try_from(value: JaResponse) -> Result<Self, Self::Error> {
        match value.janus {
            ResponseType::Event(JaHandleEvent::PluginEvent { plugin_data }) => {
                let streaming_event = from_value::<StreamingEventDto>(plugin_data.data)?;
                match streaming_event {
                    StreamingEventDto::DestroyMountpoint { id } => Ok(PluginEvent::StreamingEvent(
                        StreamingEvent::MountpointDestroyed { id },
                    )),
                    StreamingEventDto::CreateMountpoint {
                        id,
                        mountpoint_type,
                    } => Ok(PluginEvent::StreamingEvent(
                        StreamingEvent::MountpointCreated {
                            id,
                            mountpoint_type,
                        },
                    )),
                    StreamingEventDto::Event(e) => match e {
                        StreamingEventEventType::ErrorEvent { error_code, error } => {
                            Err(JaError::JanusTransport(JaTransportError::JanusError {
                                code: error_code,
                                reason: error,
                            }))
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
    use jarust_transport::japrotocol::{JaHandleEvent, JaResponse, PluginData, ResponseType};

    use super::PluginEvent;
    use crate::streaming::events::StreamingEvent;
    use crate::JanusId;

    #[test]
    fn it_parse_mountpoint_created() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.streaming".to_string(),
                    data: json!({
                        "streaming": "created",
                        "id": 6380744183070564u64,
                        "type": "live",
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
            PluginEvent::StreamingEvent(StreamingEvent::MountpointCreated {
                id: JanusId::Uint(6380744183070564u64),
                mountpoint_type: "live".to_string(),
            })
        );
    }

    #[test]
    fn it_parse_mountpoint_destroyed() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.streaming".to_string(),
                    data: json!({
                        "streaming": "destroyed",
                        "id": 6380744183070564u64,
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
            PluginEvent::StreamingEvent(StreamingEvent::MountpointDestroyed {
                id: JanusId::Uint(6380744183070564u64),
            })
        );
    }

    #[test]
    fn it_parse_error() {
        let rsp = JaResponse {
            janus: ResponseType::Event(JaHandleEvent::PluginEvent {
                plugin_data: PluginData {
                    plugin: "janus.plugin.streaming".to_string(),
                    data: json!({
                        "streaming": "event",
                        "error_code": 456,
                        "error": "Can't add 'rtp' stream, error creating data source stream"
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
        let ja_error = result.err();
        assert!(ja_error.is_some());
        assert_eq!(
            ja_error.unwrap().to_string(),
            "Transport: Janus error { code: 456, reason: Can't add 'rtp' stream, error creating data source stream}");
    }
}
