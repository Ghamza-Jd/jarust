//! # Jarust Interface
//!
//! Jarust interface continas:
//!
//! - Transport abstraction, you can use the built-in WebSocket interface, restful interface, or bring your own.
//! - Transaction generation abstraction, you can use the built-in transaction generator or bring your own.
//! - DTOs for the Janus API.
//! - Errors
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
