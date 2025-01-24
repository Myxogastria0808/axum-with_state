use crate::DashiModel;
use axum::extract::State;
use std::sync::{Arc, RwLock};

pub async fn healthcheck_handler(State(shared_state): State<Arc<RwLock<DashiModel>>>) -> String {
    let shared_model_clone = shared_state.write().unwrap();
    drop(shared_model_clone);
    "test".to_string()
}
