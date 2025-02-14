use crate::config::database::DbPool;
use crate::schema::article::dsl::*; // Import schema columns
use axum::{extract::Extension, Json};
use diesel::prelude::*;
use serde_json::json;

pub async fn get_articles(
    Extension(pool): Extension<DbPool>, // Get the connection pool
) -> Result<Json<serde_json::Value>, diesel::result::Error> {
    // Establish DB connection from pool
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Select `subTitle`, handle nulls with `Option<String>`
    let articles: Vec<Option<String>> = article
        .select(subTitle)
        .limit(1)
        .load::<Option<String>>(&mut conn)?; // Pass a mutable reference to the connection

    // Optionally, map `None` values to an empty string or some default
    let articles: Vec<String> = articles
        .into_iter()
        .map(|s| s.unwrap_or_else(|| "".to_string()))
        .collect();

    // Return the articles in JSON format
    Ok(Json(json!(articles)))
}
