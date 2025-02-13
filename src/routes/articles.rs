use crate::{config::database::DbPool, services::articles::get_articles};
use axum::{extract::Extension, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use tracing::error;

pub fn router() -> Router {
    Router::new().route("/", get(articles_handler))
}

async fn articles_handler(Extension(pool): Extension<DbPool>) -> impl IntoResponse {
    match get_articles(Extension(pool)).await {
        Ok(articles) => articles,
        Err(err) => {
            error!("Failed to fetch articles: {:?}", err);
            Json(json!({
                "error": "Failed to fetch articles"
            }))
        }
    }
}
