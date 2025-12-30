use crate::config::database::DbPool;
use crate::models::pagination::PaginationParams;
use crate::services::interview::get_interview_by_id;
use crate::services::interview::get_interview_list_paginated;

use axum::{
    extract::{Extension, Path, Query},
    routing::get,
    Json, Router,
};
use serde_json::json;
use tracing::error;

pub fn router() -> Router {
    Router::new()
        .route("/", get(interview_list_handler_paginated))
        .route("/{id}", get(interview_by_id_handler))
}

pub async fn interview_list_handler_paginated(
    Query(params): Query<PaginationParams>,
    Extension(pool): Extension<DbPool>,
) -> Json<serde_json::Value> {
    match get_interview_list_paginated(pool, params.page, params.page_size).await {
        Ok(interviews_page) => Json(json!(interviews_page)),
        Err(err) => {
            error!("Failed to fetch interviews: {:?}", err);
            Json(json!({ "error": "Failed to fetch interviews" }))
        }
    }
}

pub async fn interview_by_id_handler(
    Path(id): Path<u32>,
    Extension(pool): Extension<DbPool>,
) -> Json<serde_json::Value> {
    match get_interview_by_id(pool, id).await {
        Ok(interview) => Json(json!(interview)),
        Err(err) => {
            error!("Failed to fetch interview {}: {:?}", id, err);
            Json(json!({ "error": "Failed to fetch interview" }))
        }
    }
}
