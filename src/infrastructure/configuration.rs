use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
// use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn opendb() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(connection)
        .expect("Error connecting to database")
}
