use serde::Serialize;

create_dto!(
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
    EchoTestStartParams,
    required {
        audio: bool,
        video: bool
    },
    optional { bitrate: u32 }
);
