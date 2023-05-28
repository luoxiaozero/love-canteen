use diesel::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::user)]
pub struct User {
    pub id: i32,
    pub account: String,
    pub password: String,
    pub create_time: chrono::NaiveDateTime,
}