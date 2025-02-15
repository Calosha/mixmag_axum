use crate::config::database::DbPool;
use crate::models::article::Article;
use crate::models::article::ArticleListItem;
use crate::schema::article::dsl::*;
use diesel::prelude::*;

pub async fn get_article_list(pool: DbPool) -> Result<Vec<ArticleListItem>, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Query only necessary fields to optimize DB performance
    let articles = article
        .select((__id, title, pubDate))
        .load::<ArticleListItem>(&mut conn)?;

    Ok(articles) // ✅ Return a list of structured results
}

pub async fn get_article_by_id(
    pool: DbPool,
    article_id: u32,
) -> Result<Article, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let article_item = article
        .filter(__id.eq(article_id))
        .first::<Article>(&mut conn)?;

    Ok(article_item)
}
