# Jarust &emsp; [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/jarust.svg
[crates.io]: https://crates.io/crates/jarust

Jarust is a Rust adapter for [Janus WebRTC server](https://github.com/meetecho/janus-gateway)

Internally uses WebSockets to connect to Janus. But you can provide your own transport.

The library wraps the Janus core API and some of the most popular plugins APIs.

The supported Janus plugins currently are:

- EchoTest
- AudioBridge
- VideoBridge
- Streaming

## Examples

To run the examples first you have to lunch the janus server.

```sh
docker compose up -d
```

Then you can run any of the these examples:

- [jarust examples](./jarust/examples/), example usage of core jarust.
- [plugins examples](./jarust_plugins/examples/), example usage of jarust's predefined plugins.
