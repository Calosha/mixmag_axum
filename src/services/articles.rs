use crate::schema::article::dsl::*;
use crate::{config::database::DbPool, models::article::Article};
use axum::{extract::Extension, Json};
use diesel::prelude::*;
use serde_json::json;

pub async fn get_articles(
    Extension(pool): Extension<DbPool>,
) -> Result<Json, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let articles = article.limit(10).load::<Article>(&mut conn)?;
    Ok(Json(json!(articles)))
}
