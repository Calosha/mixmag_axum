use crate::config::database::DbPool;
use crate::models::cd_reviews::CDReview;
use crate::models::cd_reviews::CDReviewListItem;
use crate::models::pagination::PaginatedResponse;
use crate::schema::cd_review::dsl::*;
use diesel::prelude::*;

pub async fn get_cd_reviews_by_id(
    pool: DbPool,
    id: u32,
) -> Result<CDReview, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let cd_review_item = cd_review.filter(__id.eq(id)).first::<CDReview>(&mut conn)?;

    Ok(cd_review_item)
}

pub async fn get_cd_reviews_list_paginated(
    pool: DbPool,
    page: i64,
    page_size: i64,
) -> Result<PaginatedResponse<CDReviewListItem>, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let total_items = cd_review.count().get_result::<i64>(&mut conn)?;
    let total_pages = (total_items + page_size - 1) / page_size;
    let offset = (page - 1) * page_size;

    let items = cd_review
        .select((__id, title, pubDate))
        .order((pubDate.desc(), __id.desc()))
        .offset(offset)
        .limit(page_size)
        .load::<CDReviewListItem>(&mut conn)?;

    Ok(PaginatedResponse {
        items,
        total_items,
        total_pages,
        current_page: page,
    })
}
