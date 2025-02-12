use axum::{Router, routing::get};
use crate::services::health::health_check;

async fn health_handler()-> &'static str {
    "This is health"
}
pub fn router() -> Router {
    Router::new()
        .route("/", get(health_check))
}

