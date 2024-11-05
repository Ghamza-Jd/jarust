# Welcome to Jarust contributing Guide

Thank you for investing your time in helping out with Jarust's development.

If you notice you're missing any particular piece of information here, please [open an issue](https://github.com/Ghamza-Jd/jarust/issues/new) or pull request.

## Bird's-eye view of Jarust

Jarust is made up of several smaller crates - let's go over those you're most likely to interact with:

- [`jarust`](/jarust) An umbrella crate for the core, interface, and plugins crates. Typical usage of jarust should be through this crate
- [`jarust_core`](/jarust_core) Contains the high-level api of a janus adapter like creating a session, attaching, detaching, hangup, ...
- [`jarust_interface`](/jarust_interface) Contains the abstraction and the implementation of the lower level api, like network transport, transaction generation, ...
- [`jarust_plugins`](/jarust_plugins) Wraps the core and exposes a strongly typed plugin handler instead of a generic plugin
- [`jarust_rt`](/jarust_rt) Abstracts the runtime, currently only tokio is supported
- [`e2e`](/e2e) End-to-End tests, that runs test suites on a janus server and ensures nothing is broken

## Manual Testing

Manual testing is done by spinning up a docker image with janus server and pointing to it, there's a [`docker-compose`](/docker-compose.yml) file so you can run `docker compose up -d` on the root directory and by writing and running [`examples`](/jarust/examples)

## Automated Testing

- Serialization testing, it might look tedious to test (de)serialization, but the responses and events coming from janus will have different structure and fields, thanks to `serde` we could model them within the type system. But `serde` has it's complexities when we start using `flatten` with `untagged` and `tag = ""`, so serialization testing became essential to ensure a specific event will be (de)serialized to it's type counter part.

- End-to-End, the E2Es assume janus is running on the system and using the `e2e/server_config` configs, so keep that in mind when running them.
