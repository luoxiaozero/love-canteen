use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::user)]
pub struct User {
    pub id: i32,
    pub account: String,
    pub password: String,
    pub create_time: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::shop)]
pub struct Shop {
    pub id: i32,
    pub user_id: i32,
    pub create_time: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::shop_menu)]
pub struct ShopMenu {
    pub id: i32,
    pub shop_id: i32,
    pub title: String,
    pub create_time: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::shop_menu_link_food)]
pub struct ShopMenuLinkFood {
    pub id: i32,
    pub menu_id: i32,
    pub food_id: i32,
    pub create_time: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::food)]
pub struct Food {
    pub id: i32,
    pub title: String,
    pub ingredient: Text,
    pub method: Text,
    pub value: String,
    pub create_time: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::order)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub shop_id: i32,
    pub status: String,
    pub reason: String,
    pub remark: String,
    pub reserve_start_time: chrono::NaiveDateTime,
    pub reserve_end_time: chrono::NaiveDateTime,
    pub create_time: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::order_food)]
pub struct OrderFood {
    pub id: i32,
    pub food_id: i32,
    pub food_title: String,
    pub food_value: String,
    pub count: i32,
    pub create_time: chrono::NaiveDateTime,
}
