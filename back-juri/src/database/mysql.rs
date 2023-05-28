use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;
use std::env;

pub fn get_mysql_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("mysql error connecting to"))
}