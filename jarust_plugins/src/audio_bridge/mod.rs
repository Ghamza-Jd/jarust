pub mod events;
pub mod handles;
pub mod messages;

use self::events::AudioBridgePluginData;
use self::events::AudioBridgePluginEvent;
use self::handles::*;
use jarust::japrotocol::JaResponseProtocol;
use jarust::japrotocol::JaSuccessProtocol;
use jarust::prelude::*;
use jarust_make_plugin::make_plugin;

make_plugin!(AudioBridge, "janus.plugin.audiobridge");

impl AudioBridge for JaSession {
    type Event = AudioBridgePluginEvent;
    type Handle = AudioBridgeHandle;

    fn parse_audio_bridge_message(message: JaResponse) -> JaResult<Self::Event> {
        let msg = match message.janus {
            JaResponseProtocol::Success(JaSuccessProtocol::Plugin { plugin_data }) => {
                serde_json::from_value::<AudioBridgePluginData>(plugin_data)?.event
            }
            _ => {
                log::error!("unexpected response");
                return Err(JaError::UnexpectedResponse);
            }
        };
        Ok(msg)
    }
}
