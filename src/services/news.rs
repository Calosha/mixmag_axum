use crate::config::database::DbPool;
use crate::models::news::News;
use crate::models::news::NewsListItem;
use crate::models::pagination::PaginatedResponse;
use crate::schema::news::dsl::*;
use diesel::prelude::*;

pub async fn get_news_list(pool: DbPool) -> Result<Vec<NewsListItem>, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Query only necessary fields to optimize DB performance
    let news_list = news
        .select((__id, title, pubDate))
        .limit(200) // limit the number of results to 200
        .load::<NewsListItem>(&mut conn)?;

    Ok(news_list)
}

pub async fn get_news_list_paginated(
    pool: DbPool,
    page: i64,
    page_size: i64,
) -> Result<PaginatedResponse<NewsListItem>, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let total_items = news.count().get_result::<i64>(&mut conn)?;
    let total_pages = (total_items + page_size - 1) / page_size;
    let offset = (page - 1) * page_size;

    let items = news
        .select((__id, title, pubDate))
        .order((pubDate.desc(), __id.desc()))
        .offset(offset)
        .limit(page_size)
        .load::<NewsListItem>(&mut conn)?;

    Ok(PaginatedResponse {
        items,
        total_items,
        total_pages,
        current_page: page,
    })
}

pub async fn get_news_by_id(pool: DbPool, news_id: u32) -> Result<News, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let news_item = news.filter(__id.eq(news_id)).first::<News>(&mut conn)?;

    Ok(news_item)
}
