use crate::config::database::DbPool;
use crate::models::news::News;
use crate::models::news::NewsListItem;
use crate::schema::news::dsl::*;
use diesel::prelude::*;

pub async fn get_news_list(pool: DbPool) -> Result<Vec<NewsListItem>, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Query only necessary fields to optimize DB performance
    let news_list = news
        .select((__id, title, pubDate))
        .limit(200)
        .load::<NewsListItem>(&mut conn)?;

    Ok(news_list) // ✅ Return a list of structured results
}

pub async fn get_news_by_id(pool: DbPool, news_id: u32) -> Result<News, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let news_item = news.filter(__id.eq(news_id)).first::<News>(&mut conn)?;

    Ok(news_item)
}
