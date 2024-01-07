use serde::Serialize;

#[derive(Serialize)]
pub struct AudioBridgeStartMsg {
    pub audio: bool,
    pub video: bool,
}

#[derive(Serialize)]
pub struct AudioBridgeCreateMsg {
    pub request: String,
    pub room: bool,
    pub permanent: bool,
    pub description: bool,
    pub secret: bool,
    pub pin: bool,
    pub is_private: bool,
    pub allowed: bool,
    pub sampling_rate: bool,
    pub spatial_audio: bool,
    pub audiolevel_ext: bool,
    pub audiolevel_event: bool,
    pub audio_active_packets: bool,
    pub audio_level_average: bool,
    pub default_expectedloss: bool,
    pub default_bitrate: bool,
    pub record: bool,
    pub record_file: bool,
    pub record_dir: bool,
    pub mjrs: bool,
    pub mjrs_dir: bool,
    pub allow_rtp_participants: bool,
    pub groups: bool,
}

#[derive(Serialize)]
pub struct AudioBridgeListMsg {
    pub request: String,
}
impl Default for AudioBridgeListMsg {
    fn default() -> Self {
        Self {
            request: "list".to_string(),
        }
    }
}
// {
//     "request" : "create",
//     "room" : <unique numeric ID, optional, chosen by plugin if missing>,
//     "permanent" : <true|false, whether the room should be saved in the config file, default=false>,
//     "description" : "<pretty name of the room, optional>",
//     "secret" : "<password required to edit/destroy the room, optional>",
//     "pin" : "<password required to join the room, optional>",
//     "is_private" : <true|false, whether the room should appear in a list request>,
//     "allowed" : [ array of string tokens users can use to join this room, optional],
//     "sampling_rate" : <sampling rate of the room, optional, 16000 by default>,
//     "spatial_audio" : <true|false, whether the mix should spatially place users, default=false>,
//     "audiolevel_ext" : <true|false, whether the ssrc-audio-level RTP extension must be negotiated for new joins, default=true>,
//     "audiolevel_event" : <true|false (whether to emit event to other users or not)>,
//     "audio_active_packets" : <number of packets with audio level (default=100, 2 seconds)>,
//     "audio_level_average" : <average value of audio level (127=muted, 0='too loud', default=25)>,
//     "default_expectedloss" : <percent of packets we expect participants may miss, to help with FEC (default=0, max=20; automatically used for forwarders too)>,
//     "default_bitrate" : <bitrate in bps to use for the all participants (default=0, which means libopus decides; automatically used for forwarders too)>,
//     "record" : <true|false, whether to record the room or not, default=false>,
//     "record_file" : "</path/to/the/recording.wav, optional>",
//     "record_dir" : "</path/to/, optional; makes record_file a relative path, if provided>",
//     "mjrs" : <true|false (whether all participants in the room should be individually recorded to mjr files, default=false)>,
//     "mjrs_dir" : "</path/to/, optional>",
//     "allow_rtp_participants" : <true|false, whether participants should be allowed to join via plain RTP as well, default=false>,
//     "groups" : [ non-hierarchical array of string group names to use to gat participants, for external forwarding purposes only, optional]
// }
