pub mod events;
pub mod handle;
pub mod messages;

use self::events::EchoTestPluginEvent;
use self::handle::EchoTestHandle;
use crate::echotest::events::EchoTestPluginData;
use jarust::japrotocol::JaEventProtocol;
use jarust::japrotocol::JaResponseProtocol;
use jarust::prelude::*;
use jarust_make_plugin::make_plugin;

make_plugin!(EchoTest, "janus.plugin.echotest");

impl EchoTest for JaSession {
    type Event = EchoTestPluginEvent;
    type Handle = EchoTestHandle;

    fn parse_echo_test_message(message: JaResponse) -> JaResult<Self::Event> {
        let msg = match message.janus {
            JaResponseProtocol::Event(JaEventProtocol::Event { plugin_data, .. }) => {
                serde_json::from_value::<EchoTestPluginData>(plugin_data)?.event
            }
            _ => {
                log::error!("unexpected response");
                return Err(JaError::UnexpectedResponse);
            }
        };
        Ok(msg)
    }
}
