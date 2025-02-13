use crate::{
    services::articles::get_articles,
    config::database::DbPool,
};
use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    extract::Extension,
    Json,
};
use serde_json::json;

pub fn router() -> Router {
    Router::new()
        .route("/articles", get(articles_handler))
}

async fn articles_handler(
    Extension(pool): Extension<DbPool>,
) -> impl IntoResponse {
    match get_articles(Extension(pool)).await {
        Ok(articles) => articles,
        Err(_) => Json(json!({
            "error": "Failed to fetch articles"
        }))
    }
}
