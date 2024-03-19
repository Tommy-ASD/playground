use crate::{
    peer::Peer,
    room::Room,
    state::{AppState, Payload, PayloadDistribution},
};
use std::{collections::HashSet, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use traceback_error::{traceback, TracebackError};
use uuid::Uuid;

impl Room {
    #[traceback_derive::traceback]
    pub async fn add_user(&self, user_id: Uuid) -> Result<(), TracebackError> {
        for payload in self.payloads.lock().await.iter() {
            self.state
                .send(PayloadDistribution {
                    receiver_ids: vec![user_id].into_iter().collect(),
                    payload: payload.clone(),
                })
                .await?;
        }
        self.connected.lock().await.push(user_id);
        Ok(())
    }
    pub async fn remove_user(&self, user_id: Uuid) {
        let mut c = self.connected.lock().await;
        if let Some(pos) = c.iter().position(|&id| id == user_id) {
            c.remove(pos);
        }
    }
    pub async fn get_connected_ids(&self) -> HashSet<Uuid> {
        self.connected.lock().await.clone().into_iter().collect()
    }
    pub async fn get_connected(&self) -> Vec<Arc<Peer>> {
        let mut to_remove = Vec::new();
        let mut connected_peers = Vec::new();
        let mut c = self.connected.lock().await;
        for (idx, peer_id) in c.iter().enumerate() {
            if let Some(peer) = self.state.get_peer_by_id(*peer_id).await {
                connected_peers.push(peer)
            } else {
                to_remove.push(idx);
            }
        }
        to_remove.reverse();
        for idx in to_remove {
            c.remove(idx);
        }
        connected_peers
    }
}
