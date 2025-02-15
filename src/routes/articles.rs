use crate::{
    config::database::DbPool, services::articles::get_article_by_id,
    services::articles::get_article_list,
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
        .route("/", get(article_list_handler))
        .route("/{id}", get(article_by_id_handler))
}

pub async fn article_list_handler(Extension(pool): Extension<DbPool>) -> Json<serde_json::Value> {
    match get_article_list(pool).await {
        Ok(articles) => Json(json!(articles)),
        Err(err) => {
            error!("Failed to fetch article list: {:?}", err);
            Json(json!({ "error": "Failed to fetch articles" }))
        }
    }
}

pub async fn article_by_id_handler(
    Path(id): Path<u32>,
    Extension(pool): Extension<DbPool>,
) -> Json<serde_json::Value> {
    match get_article_by_id(pool, id).await {
        Ok(article) => Json(json!(article)),
        Err(err) => {
            error!("Failed to fetch article: {:?}", err);
            Json(json!({ "error": "Failed to fetch article" }))
        }
    }
}
