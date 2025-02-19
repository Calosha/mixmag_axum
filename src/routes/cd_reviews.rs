use crate::models::pagination::PaginationParams;
use crate::services::cd_reviews::get_cd_reviews_by_id;
use crate::{config::database::DbPool, services::cd_reviews::get_cd_reviews_list_paginated};
use axum::{
    extract::{Extension, Path, Query},
    routing::get,
    Json, Router,
};
use serde_json::json;
use tracing::error;

pub fn router() -> Router {
    Router::new()
        .route("/", get(cd_review_list_handler_paginated))
        .route("/{id}", get(cd_review_by_id_handler))
}

pub async fn cd_review_list_handler_paginated(
    Query(params): Query<PaginationParams>,
    Extension(pool): Extension<DbPool>,
) -> Json<serde_json::Value> {
    match get_cd_reviews_list_paginated(pool, params.page, params.page_size).await {
        Ok(news_page) => Json(json!(news_page)),
        Err(err) => {
            error!("Failed to fetch news: {:?}", err);
            Json(json!({ "error": "Failed to fetch news" }))
        }
    }
}

pub async fn cd_review_by_id_handler(
    Path(id): Path<u32>,
    Extension(pool): Extension<DbPool>,
) -> Json<serde_json::Value> {
    match get_cd_reviews_by_id(pool, id).await {
        Ok(news_page) => Json(json!(news_page)),
        Err(err) => {
            error!("Failed to fetch news: {:?}", err);
            Json(json!({ "error": "Failed to fetch news" }))
        }
    }
}
