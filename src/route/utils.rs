use std::sync::{Arc, RwLock};

use crate::{DashiModel, handler::utils::healthcheck_handler};
use axum::{Router, routing::get};

pub fn util_route() -> Router<Arc<RwLock<DashiModel>>> {
    let utils_routes: Router<Arc<RwLock<DashiModel>>> =
        Router::new().route("/healthcheck", get(healthcheck_handler));
    Router::new().nest("/utils", utils_routes)
}
