use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub fn make_router() -> Router<Arc<AppState>> {
    Router::new()
}
