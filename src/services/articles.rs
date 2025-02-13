use crate::schema::article::dsl::*;
use crate::{config::database::DbPool, models::article::Article};
use diesel::prelude::*;

pub async fn get_articles(pool: &DbPool) -> Result<Vec<Article>, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    article.limit(10).load::<Article>(&mut conn)
}
