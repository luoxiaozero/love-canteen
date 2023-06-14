use diesel::{Connection, MysqlConnection, RunQueryDsl};
use dotenv::dotenv;
use std::env;

pub fn get_mysql_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut conn = MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("mysql error connecting to"));

    _ = diesel::sql_query("SET time_zone = '+08:00';").execute(&mut conn).expect("set time zone");
    conn
}