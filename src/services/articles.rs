use crate::schema::article::dsl::*;
use crate::{config::database::DbPool, models::article::Article};
use axum::response::Json;
use diesel::prelude::*;
use serde_json::json;

use crate::schema::article::dsl::*;
use diesel::prelude::*;

pub async fn get_articles(
    Extension(pool): Extension<DbPool>,
) -> Result<Json, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let articles = article.limit(10).load::<Article>(&mut conn)?;
    Ok(Json(json!(articles)))
}
