use crate::services::health::health_check;
use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/", get(health_check))
}
