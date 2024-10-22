make_dto!(
    EchoTestStartParams,
    optional {
        audio: bool,
        video: bool,
        bitrate: u32,
        record: bool,
        filename: String,
        substream: u32,
        temporal: u32,
        fallback: u32,
        svc: bool,
        spatial_layer: u32,
        temporal_layer: u32,
        audiocodec: String,
        videocodec: String,
        videoprofile: String,
        opusred: bool,
        min_delay: i32,
        max_delay: i32,
    }
);
