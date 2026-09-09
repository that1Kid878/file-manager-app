mod health;
use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{api::health::health_check, core::state::AppState};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(state)
}
