use crate::{config::database::DbPool, services::articles::get_articles};
use axum::{extract::Extension, routing::get, Json, Router};
use serde_json::json;
use tracing::error;

pub fn router() -> Router {
    Router::new().route("/", get(articles_handler))
}

pub async fn articles_handler(Extension(pool): Extension<DbPool>) -> Json<serde_json::Value> {
    match get_articles(pool).await {
        Ok(articles) => Json(json!(articles)), // Wrap articles in Json<serde_json::Value>
        Err(err) => {
            error!("Failed to fetch articles: {:?}", err);
            Json(json!({
                "error": "Failed to fetch articles"
            })) // Return error message as Json<serde_json::Value>
        }
    }
}
