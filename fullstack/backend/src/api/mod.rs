use axum::Router;

use serde::{Deserialize, Serialize};

pub fn make_router<T>() -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    Router::new()
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct User {
    pub username: String,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub sender: String, // refers to username
}
