//! Internal Copilot SDK - JSON-RPC client for the Copilot CLI.
//!
//! Derived from the official [github/copilot-sdk](https://github.com/github/copilot-sdk)
//! Go implementation. Communicates with `copilot --headless --stdio` via JSON-RPC 2.0.

mod jsonrpc;
mod types;
mod client;
mod session;

pub use client::{Client, ClientOptions};
pub use session::Session;
pub use types::*;
