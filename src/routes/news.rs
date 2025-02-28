use crate::models::pagination::PaginationParams;
use crate::{
    config::database::DbPool, services::news::get_news_by_id, services::news::get_news_list,
    services::news::get_news_list_paginated,
};
use axum::{
    extract::{Extension, Path, Query},
    routing::get,
    Json, Router,
};
use serde_json::json;
use tracing::error;

pub fn router() -> Router {
    Router::new()
        //TODO: Add overflow protection for page and page_size
        .route("/", get(news_list_handler_paginated))
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

pub async fn news_list_handler_paginated(
    Query(params): Query<PaginationParams>,
    Extension(pool): Extension<DbPool>,
) -> Json<serde_json::Value> {
    match get_news_list_paginated(pool, params.page, params.page_size).await {
        Ok(news_page) => Json(json!(news_page)),
        Err(err) => {
            error!("Failed to fetch news: {:?}", err);
            Json(json!({ "error": "Failed to fetch news" }))
        }
    }
}
