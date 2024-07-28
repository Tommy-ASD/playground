use serde::{Deserialize, Serialize};

pub mod from_client;
pub mod from_server;

pub use self::{from_client::ClientPayload, from_server::ServerPayload};
