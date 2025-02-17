use crate::{
    config::database::DbPool, services::news::get_news_by_id, services::news::get_news_list,
};
use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use serde_json::json;
use tracing::error;

pub fn router() -> Router {
    Router::new()
        .route("/", get(news_list_handler))
        .route("/{id}", get(news_by_id_handler))
}

pub async fn news_list_handler(Extension(pool): Extension<DbPool>) -> Json<serde_json::Value> {
    match get_news_list(pool).await {
        Ok(news) => Json(json!(news)),
        Err(err) => {
            error!("Failed to fetch news list: {:?}", err);
            Json(json!({ "error": "Failed to fetch news" }))
        }
    }
}

pub async fn news_by_id_handler(
    Path(id): Path<u32>,
    Extension(pool): Extension<DbPool>,
) -> Json<serde_json::Value> {
    match get_news_by_id(pool, id).await {
        Ok(news) => Json(json!(news)),
        Err(err) => {
            error!("Failed to fetch news: {:?}", err);
            Json(json!({ "error": "Failed to fetch news" }))
        }
    }
}
