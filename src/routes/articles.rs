use crate::{config::database::DbPool, services::articles::get_article_list};
use axum::{extract::Extension, routing::get, Json, Router};
use serde_json::json;
use tracing::error;

pub fn router() -> Router {
    Router::new().route("/", get(article_list_handler)) // ✅ List route
}

pub async fn article_list_handler(Extension(pool): Extension<DbPool>) -> Json<serde_json::Value> {
    match get_article_list(pool).await {
        Ok(articles) => Json(json!(articles)), // ✅ Return structured JSON
        Err(err) => {
            error!("Failed to fetch article list: {:?}", err);
            Json(json!({ "error": "Failed to fetch articles" }))
        }
    }
}
