use std::sync::Arc;

pub struct AppState {}

pub fn create_state() -> Arc<AppState> {
    return Arc::new(AppState {});
}
