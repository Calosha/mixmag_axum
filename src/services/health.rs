use axum::{
    extract::Extension,
    Json,
};
use diesel::RunQueryDsl;
use serde_json::json;
use crate::config::database::DbPool;

pub async fn health_check(
    Extension(pool): Extension<DbPool>,
) -> Json<serde_json::Value> {
    let db_status = match pool.get() {
        Ok(mut conn) => {
            use diesel::dsl::*;
            match sql::<diesel::sql_types::Integer>("SELECT 1")
                .execute(&mut conn)
            {
                Ok(_) => "healthy",
                Err(_) => "unhealthy",
            }
        },
        Err(_) => "unhealthy",
    };

    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "database": db_status,
    }))
}
