use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::article)]
pub struct Article {
    pub __id: u32,
    pub __status: u8,
    pub __created: u32,
    pub __updated: u32,
    pub sub_title: Option<String>,
    pub img_wide: Option<String>,
    pub img_top: Option<String>,
    pub is_main: Option<u8>,
    pub announcement_main: Option<String>,
    pub img_main: Option<String>,
    pub is_black: Option<u8>,
    pub lj_id: Option<u32>,
    pub title: String,
    pub announcement: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<u8>>,
    pub uid: Option<u32>,
    pub views: Option<u32>,
    pub hash: Option<String>,
    pub comment_access: Option<u8>,
    pub ccount: Option<u32>,
    pub pub_date: Option<u32>,
    pub rate: Option<u32>,
    pub rate_positive: Option<u32>,
    pub rate_negative: Option<u32>,
    pub rated_list: Option<Vec<u8>>,
}
