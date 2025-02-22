use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::cd_review)]
pub struct CDReview {
    #[diesel(column_name = __id)]
    pub id: u32,
    #[diesel(column_name = __status)]
    pub status: u8,
    #[diesel(column_name = __created)]
    pub created: u32,
    #[diesel(column_name = __updated)]
    pub updated: u32,
    #[diesel(column_name = nameId)]
    pub name_id: Option<u32>,
    pub title: Option<String>,
    #[diesel(column_name = labelList)]
    pub label_list: Option<Vec<u8>>,
    #[diesel(column_name = genreList)]
    pub genre_list: Option<Vec<u8>>,
    pub tags: Option<Vec<u8>>,
    pub year: Option<i16>,
    #[diesel(column_name = trackListUrl)]
    pub track_list_url: Option<String>,
    #[diesel(column_name = trackList)]
    pub track_list: Option<Vec<u8>>,
    pub description: Option<String>,
    #[diesel(column_name = imgBig)]
    pub img_big: Option<String>,
    pub uid: Option<u32>,
    pub views: Option<u32>,
    pub hash: Option<String>,
    pub ccount: Option<u32>,
    #[diesel(column_name = pubDate)]
    pub pub_date: Option<u32>,
    pub rate: Option<u32>,
    #[diesel(column_name = ratePositive)]
    pub rate_positive: Option<u32>,
    #[diesel(column_name = rateNegative)]
    pub rate_negative: Option<u32>,
    #[diesel(column_name = ratedList)]
    pub rated_list: Option<Vec<u8>>,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::cd_review)]
pub struct CDReviewListItem {
    #[diesel(column_name = __id)]
    pub id: u32,
    pub title: Option<String>,
    #[diesel(column_name = pubDate)]
    pub pub_date: Option<u32>,
}

#[derive(Serialize)]
pub struct CDReviewListItemWithName {
    pub id: u32,
    pub title: Option<String>,
    pub pub_date: Option<u32>,
    pub name: Option<String>,
}
