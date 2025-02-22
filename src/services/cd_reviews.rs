use crate::config::database::DbPool;
use crate::models::cd_reviews::CDReview;
use crate::models::cd_reviews::CDReviewListItemWithName;
use crate::models::pagination::PaginatedResponse;
use crate::schema::cd_review::dsl::__id as cd_review_id;
use crate::schema::cd_review::dsl::cd_review;
use crate::schema::cd_review::dsl::nameId as cd_review_name_id;
use crate::schema::cd_review::dsl::pubDate;
use crate::schema::cd_review::dsl::title;
use crate::schema::name::dsl::__id as name_id;
use crate::schema::name::dsl::name;
use crate::schema::name::dsl::name_col;
use diesel::prelude::*;

pub async fn get_cd_reviews_by_id(
    pool: DbPool,
    id: u32,
) -> Result<CDReview, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let cd_review_item = cd_review
        .filter(cd_review_id.eq(id))
        .first::<CDReview>(&mut conn)?;

    Ok(cd_review_item)
}

pub async fn get_cd_reviews_list_paginated(
    pool: DbPool,
    page: i64,
    page_size: i64,
) -> Result<PaginatedResponse<CDReviewListItemWithName>, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let total_items = cd_review
        .select(cd_review_id)
        .count()
        .get_result::<i64>(&mut conn)?;

    let total_pages = (total_items + page_size - 1) / page_size;
    let offset = (page - 1) * page_size;

    let items = cd_review
        .inner_join(name.on(cd_review_name_id.eq(name_id.nullable())))
        .select((cd_review_id, title, pubDate, name_col))
        .order((pubDate.desc(), cd_review_id.desc()))
        .offset(offset)
        .limit(page_size)
        .load::<(u32, Option<String>, Option<u32>, Option<String>)>(&mut conn)?
        .into_iter()
        .map(
            |(id, title_res, pub_date_res, name_res)| CDReviewListItemWithName {
                id: id,
                title: title_res,
                pub_date: pub_date_res,
                name: name_res,
            },
        )
        .collect::<Vec<CDReviewListItemWithName>>();

    Ok(PaginatedResponse {
        items,
        total_items,
        total_pages,
        current_page: page,
    })
}
