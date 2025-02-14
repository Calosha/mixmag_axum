use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = crate::schema::article)]
pub struct Article {
    #[diesel(column_name = __id)]
    pub id: u32,
    #[diesel(column_name = __status)]
    pub status: u8,
    #[diesel(column_name = __created)]
    pub created: u32,
    #[diesel(column_name = __updated)]
    pub updated: u32,
    #[diesel(column_name = subTitle)]
    pub sub_title: Option<String>,
    #[diesel(column_name = imgWide)]
    pub img_wide: Option<String>,
    #[diesel(column_name = imgTop)]
    pub img_top: Option<String>,
    #[diesel(column_name = isMain)]
    pub is_main: Option<u8>,
    #[diesel(column_name = announcementMain)]
    pub announcement_main: Option<String>,
    #[diesel(column_name = imgMain)]
    pub img_main: Option<String>,
    #[diesel(column_name = isBlack)]
    pub is_black: Option<u8>,
    #[diesel(column_name = ljId)]
    pub lj_id: Option<u32>,
    pub title: String,
    pub announcement: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<u8>>,
    pub uid: Option<u32>,
    pub views: Option<u32>,
    pub hash: Option<String>,
    #[diesel(column_name = commentAccess)]
    pub comment_access: Option<u8>,
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
#[diesel(table_name = crate::schema::article)]
pub struct ArticleListItem {
    pub title: String,
    #[diesel(sql_type = Unsigned<Integer>)]
    #[diesel(column_name = pubDate)]
    pub pub_date: Option<u32>,
}
