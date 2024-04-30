pub mod events;
pub mod handles;
pub mod messages;
pub mod results;

use self::events::AudioBridgePluginData;
use self::events::AudioBridgePluginEvent;
use self::handles::*;
use jarust::japrotocol::EstablishmentProtocol;
use jarust::japrotocol::JaEventProtocol;
use jarust::japrotocol::JaResponseProtocol;
use jarust::prelude::*;
use jarust_make_plugin::make_plugin;

make_plugin!(AudioBridge, "janus.plugin.audiobridge");

impl AudioBridge for JaSession {
    type Event = (AudioBridgePluginEvent, Option<EstablishmentProtocol>);
    type Handle = AudioBridgeHandle;

    fn parse_audio_bridge_message(message: JaResponse) -> JaResult<Self::Event> {
        let msg = match message.janus {
            JaResponseProtocol::Event(JaEventProtocol::Event { plugin_data }) => (
                serde_json::from_value::<AudioBridgePluginData>(plugin_data.data)?.event,
                message.establishment_protocol,
            ),
            _ => {
                tracing::error!("unexpected response");
                return Err(JaError::UnexpectedResponse);
            }
        };
        Ok(msg)
    }
}
