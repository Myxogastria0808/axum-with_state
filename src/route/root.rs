use std::sync::{Arc, RwLock};

use crate::DashiModel;
use crate::route::utils::util_route;
use axum::Router;

pub fn root_route() -> Router<Arc<RwLock<DashiModel>>> {
    let root_routes: Router<Arc<RwLock<DashiModel>>> = Router::new().merge(util_route());
    Router::new().nest("/api", root_routes)
}
