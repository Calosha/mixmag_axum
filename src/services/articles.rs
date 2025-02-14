use crate::config::database::DbPool;
use crate::schema::article::dsl::*; // Import schema columns
use diesel::prelude::*;

pub async fn get_articles(pool: DbPool) -> Result<Vec<String>, diesel::result::Error> {
    // Establish DB connection from pool
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Query the database for article titles
    let articles: Vec<String> = article
        .select(title) // Select only the `title` field
        .load::<String>(&mut conn)?; // Load the query result into a Vec<String>

    // Return the list of titles
    Ok(articles)
}
