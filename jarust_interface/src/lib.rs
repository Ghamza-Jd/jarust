//! # Jarust Interface
//!
//! Jarust Interface abstracts the implementation details of the connection and the underlying transport, allowing
//! customization of the transport layer. For example, dealing with WebSockets on major platforms can be over TCP, while in
//! browsers, we have to use something like the web_sys crate.
//!
//! It also enables choosing one of the built-in interfaces or bringing your own interface. The same goes for transaction generation.
//!

pub mod error;
pub mod handle_msg;
pub mod janus_interface;
pub mod japrotocol;
pub mod respones;
pub mod restful;
pub mod tgenerator;
pub mod websocket;

pub type Error = error::Error;
