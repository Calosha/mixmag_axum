use crate::config::database::DbPool;
use crate::models::interview::Interview;
use crate::models::interview::InterviewListItem;
use crate::models::pagination::PaginatedResponse;
use crate::schema::interview::dsl::*;
use diesel::prelude::*;

pub async fn get_interview_list(
    pool: DbPool,
) -> Result<Vec<InterviewListItem>, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Query only necessary fields to optimize DB performance
    let interviews = interview
        .select((__id, title, pubDate))
        .load::<InterviewListItem>(&mut conn)?;

    Ok(interviews) // ✅ Return the loaded interviews list
}

pub async fn get_interview_list_paginated(
    pool: DbPool,
    page: i64,
    page_size: i64,
) -> Result<PaginatedResponse<InterviewListItem>, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let total_items = interview.count().get_result::<i64>(&mut conn)?;
    let total_pages = (total_items + page_size - 1) / page_size;
    let offset = (page - 1) * page_size;

    let items = interview
        .select((__id, title, pubDate))
        .order((pubDate.desc(), __id.desc()))
        .offset(offset)
        .limit(page_size)
        .load::<InterviewListItem>(&mut conn)?;

    Ok(PaginatedResponse {
        items,
        total_items,
        total_pages,
        current_page: page,
    })
}

pub async fn get_interview_by_id(
    pool: DbPool,
    interview_id: u32,
) -> Result<Interview, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let interview_item = interview
        .filter(__id.eq(interview_id))
        .first::<Interview>(&mut conn)?;

    Ok(interview_item)
}
