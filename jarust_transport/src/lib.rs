//! # Jarust Transport
//!
//! Jarust transport abstracts the implementation details of the connection and the underlying transport which allows
//! customization of the transport layer. For example, dealing with WebSockets on major platforms can be over TCP while on
//! browser we have to use something like web_sys crate.
//!
//! It also enables choosing one of the built-in interfaces or bringing your own interface. And the same goes for the transaction generation.
//!

pub mod error;
pub mod handle_msg;
pub mod janus_interface;
pub mod japrotocol;
pub mod prelude;
pub mod respones;
pub mod restful;
pub mod tgenerator;
pub mod websocket;
