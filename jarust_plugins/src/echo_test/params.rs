make_dto!(
    EchoTestStartParams,
    required {
        audio: bool,
        video: bool
    },
    optional { bitrate: u32 }
);
