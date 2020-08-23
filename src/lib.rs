mod newsinboundr_db {
    use diesel::{PgConnection, RunQueryDsl};
    use diesel::result::Error;
    use crate::db::models::NewNews;
    use crate::db::schema::news;
    use crate::db::schema::NewsSourceType;
    use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};

    pub fn create_news<'a>(conn: &PooledConnection<ConnectionManager<PgConnection>>,
                           guid: &'a str,
                           source_type: &'a NewsSourceType,
                           source_id: &'a i32,
                           insert_date: &'a chrono::NaiveDateTime,
                           title: &'a str,
                           content: &'a str,
                           link: &'a str,
                           color: &'a str) -> Result<usize, Error> {
        let new_news = NewNews {
            guid,
            source_type,
            source_id,
            insert_date,
            title,
            content,
            link,
            color
        };

        diesel::insert_into(news::table)
            .values(&new_news)
            .execute(conn)
    }


    pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager =
            ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder().build(manager).expect("Error building pool")
    }


    pub fn get_atom_sources(connection: PooledConnection<ConnectionManager<PgConnection>>) -> Option<Vec<SourceAtom>> {
        return None;
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
}