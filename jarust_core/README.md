# Jarust Core

The core of jarust.

It under the hood it uses [jarust_interface](https://crates.io/crates/jarust_interface) to provide an abstract api
for connecting, creating a session, attaching to a plugin, and then communicate with the plugin handle.

It's also the building block for the plugin crate [jarust_plugins](https://crates.io/crates/jarust_plugins)
