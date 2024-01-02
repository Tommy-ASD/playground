use futures::future::join_all;
use std::{collections::HashSet, sync::Mutex};
use tokio::sync::broadcast::{self, Receiver};

use serde_json::json;

use traceback_error::{traceback, TracebackError};

use common::{Payload, User};

#[derive(Clone)]
pub struct Sender {
    pub inner: broadcast::Sender<Payload>,
}

impl Sender {
    #[traceback_derive::traceback]
    fn send(&self, payload: Payload) -> Result<(), TracebackError> {
        self.inner.send(payload)?;

        Ok(())
    }
    pub fn subscribe(&self) -> Receiver<Payload> {
        self.inner.subscribe()
    }
}

// Our shared state
pub struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    pub user_set: Mutex<HashSet<User>>,
    // Channel used to send payloads to all connected clients.
    pub tx: Sender,
    // store payloads
    pub payloads: Mutex<Vec<Payload>>,
}

impl AppState {
    pub async fn username_is_unique(&self, name: &str) -> bool {
        let res = join_all(
            self.user_set
                .lock()
                .unwrap()
                .iter()
                .map(|user| async move { user.get_username().await == name }),
        )
        .await;

        res.into_iter().any(|found| found)
    }
    /// Returns none if username is already taken
    pub async fn add_user(&self, name: &str) -> Option<User> {
        if self.username_is_unique(name).await {
            let user = User::new(name);
            self.user_set.lock().unwrap().insert(user.clone());
            return Some(user);
        }
        None
    }
    #[traceback_derive::traceback]
    pub fn send(&self, payload: Payload) -> Result<(), TracebackError> {
        tracing::debug!("Sending {payload:?}");
        self.tx.send(payload.clone())?;
        let mut payloads = match self.payloads.lock() {
            Ok(payloads) => payloads,
            Err(e) => {
                return Err(
                    TracebackError::new(String::from(""), file!().to_string(), line!())
                        .with_extra_data(json!({
                            "error": e.to_string()
                        })),
                );
            }
        };
        payloads.push(payload.clone());

        Ok(())
    }
    #[traceback_derive::traceback]
    pub fn get_payload_list(&self) -> Result<Vec<Payload>, TracebackError> {
        match self.payloads.lock() {
            Ok(pl) => Ok(pl.iter().map(|pl| pl.clone()).collect::<Vec<Payload>>()),
            Err(e) => Err(traceback!().with_extra_data(json!({"error": e.to_string()}))),
        }
    }
}
