mod food;
mod menu;

use crate::utils::user::get_user_info;
use crate::{
    database::{
        models::{NewShop, Shop},
        mysql::get_mysql_connection,
        schema::shop,
    },
    utils::{ext::ResulResponseExt, result_response},
};
use diesel::prelude::*;
use juri::Router;
use juri::{handler, Request, Response};
use serde_json::{json, Value};

pub fn router() -> Router {
    let mut router = Router::new();
    router.at("/shop").get(get_shop_vec);
    router.at("/shop/exist").get(is_exist_shop);
    router.at("/shop/create").post(create_shop);
    router.at("/shop/menu").get(menu::get_menu);
    router.at("/shop/menu/add").post(menu::add_menu);
    router.at("/shop/menu/food").get(food::get_menu_food);
    router.at("/shop/food/add").post(food::add_food);
    router
}

#[handler]
pub fn get_shop_vec(request: &Request) -> juri::Result<Response> {
    let _user = get_user_info(request.header("token"))?;

    let conn = &mut get_mysql_connection();
    let shop_vec: Vec<Shop> = shop::table.load(conn).ok_or_status_4001()?;

    let shop_vec_json: Vec<Value> = shop_vec
        .iter()
        .map(|shop| {
            json!({
                "id": shop.id,
            })
        })
        .collect();
    let data = json!(shop_vec_json);
    Ok(result_response::success_data("获取成功", &data)?)
}

#[handler]
pub fn is_exist_shop(request: &Request) -> juri::Result<Response> {
    let user = get_user_info(request.header("token"))?;

    let conn = &mut get_mysql_connection();
    let shop_vec: Vec<Shop> = shop::table
        .filter(shop::user_id.eq(user.id))
        .limit(1)
        .load(conn)
        .ok_or_status_4001()?;

    if shop_vec.is_empty() {
        return Ok(result_response::success_data("获取成功", &false)?);
    }

    Ok(result_response::success_data("获取成功", &true)?)
}

#[handler]
pub fn create_shop(request: &Request) -> juri::Result<Response> {
    let user = get_user_info(request.header("token"))?;

    let conn = &mut get_mysql_connection();
    let shop_vec: Vec<Shop> = shop::table
        .filter(shop::user_id.eq(user.id))
        .limit(1)
        .load(conn)
        .ok_or_status_4001()?;

    if shop_vec.is_empty() {
        let new_shop = NewShop { user_id: user.id };
        diesel::insert_into(shop::table)
            .values(&new_shop)
            .execute(conn)
            .ok_or_status_4001()?;
        return Ok(result_response::status_2000("创建成功")?);
    }

    Ok(result_response::status_2000("成功")?)
}
