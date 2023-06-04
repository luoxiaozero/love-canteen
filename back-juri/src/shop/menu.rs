use crate::utils::user::get_user_info;
use crate::{
    database::{
        models::{NewShopMenu, Shop, ShopMenu},
        mysql::get_mysql_connection,
        schema::{shop, shop_menu},
    },
    utils::{
        ext::{OptionResponseExt, ResulResponseExt},
        result_response,
    },
};
use diesel::prelude::*;
use juri::{handler, json::JsonRequestExt, Request, Response};
use serde_json::{json, Value};

#[handler]
pub fn get_menu(request: &Request) -> juri::Result<Response> {
    let user = get_user_info(request.header("token"))?;

    let conn = &mut get_mysql_connection();
    let shop: Shop = shop::table
        .filter(shop::user_id.eq(user.id))
        .first(conn)
        .ok_or_status_4001()?;

    let shop_menu_vec: Vec<ShopMenu> = shop_menu::table
        .filter(shop_menu::shop_id.eq(shop.id))
        .load(conn)
        .ok_or_status_4001()?;

    let shop_menu_vec_json: Vec<Value> = shop_menu_vec
        .iter()
        .map(|shop_menu| {
            json!({
                "id": shop_menu.id,
                "title": shop_menu.title,
            })
        })
        .collect();
    let data = json!(shop_menu_vec_json);
    Ok(result_response::success_data("获取成功", &data)?)
}

#[handler]
pub fn add_menu(request: &Request) -> juri::Result<Response> {
    let user = get_user_info(request.header("token"))?;
    let body_json: Value = request.json_value().ok_or_status_4001()?;
    let title = body_json["title"].as_str().ok_or_status_4001()?;

    let conn = &mut get_mysql_connection();
    let shop: Shop = shop::table
        .filter(shop::user_id.eq(user.id))
        .first(conn)
        .ok_or_status_4001()?;

    let new_menu = NewShopMenu {
        shop_id: shop.id,
        title: title.to_string(),
    };
    diesel::insert_into(shop_menu::table)
        .values(&new_menu)
        .execute(conn)
        .ok_or_status_4001()?;
    let menu: ShopMenu = shop_menu::table
        .order(shop_menu::id.desc())
        .first(conn)
        .ok_or_status_4001()?;

    let data = json!({"id": menu.id, "title": menu.title });
    Ok(result_response::success_data("添加成功", &data)?)
}
