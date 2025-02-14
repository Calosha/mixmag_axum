use crate::config::database::DbPool;
use crate::models::article::ArticleListItem;
use crate::schema::article::dsl::*;
use diesel::prelude::*;
use diesel::sql_query; // Add this for raw SQL support
use diesel::sql_types::Text; // Add this for our diagnostic type

// We'll create a temporary struct just for debugging
#[derive(QueryableByName, Debug)]
struct DiagnosticArticle {
    #[diesel(sql_type = Text)]
    pub article_title: String,
    #[diesel(sql_type = Text)]
    pub debug_info: String,
}

pub async fn get_article_list(pool: DbPool) -> Result<Vec<ArticleListItem>, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Instead of your original query, we'll use a diagnostic query that's similar in structure
    let diagnostic_results = sql_query(
        "
        SELECT 
            title as article_title,
            CONCAT('PubDate raw value: ', CAST(pubDate AS CHAR)) as debug_info
        FROM article
        LIMIT 5
    ",
    )
    .load::<DiagnosticArticle>(&mut conn)?;

    // Print the diagnostic information
    for result in &diagnostic_results {
        println!("Title: {}", result.article_title);
        println!("Debug Info: {}", result.debug_info);
        println!("-----------------");
    }

    // Now try your original query
    let articles = article
        .select((title, pubDate))
        .load::<ArticleListItem>(&mut conn)?;

    Ok(articles)
}
