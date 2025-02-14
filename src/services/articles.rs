use crate::config::database::DbPool;
use crate::models::article::ArticleListItem; // ✅ Import our new struct
use crate::schema::article::dsl::*; // Import schema columns
use diesel::prelude::*;

pub async fn get_article_list(pool: DbPool) -> Result<Vec<ArticleListItem>, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Query only necessary fields to optimize DB performance
    let articles = article
        .select((__id, title, pubDate))
        .load::<ArticleListItem>(&mut conn)?;

    Ok(articles) // ✅ Return a list of structured results
}
