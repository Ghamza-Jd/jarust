# Jarust &emsp; [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/jarust.svg
[crates.io]: https://crates.io/crates/jarust

Jarust is a memory safe and high-performance Rust adapter for [Janus WebRTC server](https://github.com/meetecho/janus-gateway).
Inspired by [Janode](https://github.com/meetecho/janode), jarust offers similar functionalities but it's designed
to be customizable, for exmaple, you could use the built-in WebSocket transport or provide your own RabbitMQ transport implementation.

The library wraps the Janus core API and some of the most popular plugins APIs.

The supported Janus plugins currently are:

- EchoTest
- AudioBridge
- Streaming
- VideoRoom

The supported interfaces are:

- WebSocket
- Restful

## Examples

To run the examples first you have to lunch the janus server.

```sh
docker compose up -d
```

Then you can run any of the these examples:

- [jarust examples](./jarust/examples/), example usage of core jarust.
- [plugins examples](./jarust_plugins/examples/), example usage of jarust's predefined plugins.
