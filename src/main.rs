use axum::{Router, routing::get};
use std::sync::{Arc, RwLock};

mod handler;
mod route;

#[tokio::main]
async fn main() {
    let _ = api().await;
}

#[derive(Debug, Clone)]
pub struct DashiModel {}

//axum
async fn api() -> Result<(), axum::Error> {
    //Shared Object
    let shared_state: Arc<RwLock<DashiModel>> = Arc::new(RwLock::new(DashiModel {}));
    //Router
    //* ここの型を`Router<()>`にする理由がわからず、困っています...。 *//
    let app: Router<()> = Router::new()
        .route("/", get(ping))
        .merge(route::root::root_route())
        .with_state(Arc::clone(&shared_state));
    //Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

//* dummy *//
async fn ping() -> String {
    "pong!".to_string()
}
