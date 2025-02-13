use crate::services::articles::get_articles;
use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/", get(get_articles))
}
