#[macro_use]
extern crate diesel;

mod db;

use diesel::{PgConnection, RunQueryDsl};
use diesel::result::Error;
use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};
use db::models::{ NewNews, SourceAtom, SourceRss};
use db::schema::{news, NewsSourceType};
use db::schema::source_atom::dsl::*;
use db::schema::source_rss::dsl::*;

pub fn create_news<'a>(conn: &PooledConnection<ConnectionManager<PgConnection>>,
                       guid: &'a str,
                       source_type: &'a NewsSourceType,
                       source_id: &'a i32,
                       insert_date: &'a chrono::NaiveDateTime,
                       title: &'a str,
                       content: &'a str,
                       link: &'a str,
                       i_color: &'a str) -> Result<usize, Error> {

    let new_news = NewNews{
        guid,
        source_type,
        source_id,
        insert_date,
        title,
        content,
        link,
        color: i_color
    };

    diesel::insert_into(news::table)
        .values(&new_news)
        .execute(conn)
}


pub fn establish_connection(database_url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager =
        ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Error building pool")
}


pub fn get_atom_sources(connection: PooledConnection<ConnectionManager<PgConnection>>) -> Option<Vec<SourceAtom>> {
    let results = source_atom
        .load(&connection)
        .expect("Error loading posts");

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

pub fn get_rss_sources(connection: PooledConnection<ConnectionManager<PgConnection>>) -> Option<Vec<SourceRss>> {

    let results = source_rss
        .load(&connection)
        .expect("Error loading posts");

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}