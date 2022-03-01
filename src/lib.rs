#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod database;

use database::model::{NewPost, Post, Sequences};
use database::schema::posts::{id, table as posts};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn insert_posts<'a>(conn: &MysqlConnection, name: &'a str) -> i64 {
    let new_posts = NewPost { name };
    diesel::insert_into(posts)
        .values(&new_posts)
        .execute(conn)
        .expect("Error saving new posts");
    let generate_id = diesel::sql_query("select LAST_INSERT_ID() as id")
        .load::<Sequences>(conn)
        .expect("get_id_err")
        .first()
        .unwrap()
        .id;
    generate_id
}

pub fn query_posts_by_id<'a>(conn: &MysqlConnection, post_id: &'a i64) -> Post {
    let post = posts
        .filter(id.eq_all(post_id))
        .first::<Post>(conn)
        .expect("query post fail");
    post
}

pub fn update_posts_by_id<'a>(
    conn: &MysqlConnection,
    post_id: &'a i64,
    post_name: &'a &str,
) -> Result<(), Error> {
    // todo: update post by id
    Ok(())
}
