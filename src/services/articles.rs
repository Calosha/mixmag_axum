use crate::{config::database::DbPool, models::article::Article};
use diesel::prelude::*;

pub async fn get_articles(pool: &DbPool) -> Result<Vec<Article>, diesel::result::Error> {
    use crate::schema::articles::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection");
    articles.limit(10).load::<Article>(&mut conn)
}
