use crate::database::models::NewShopMenuLinkFood;
use crate::utils::user::get_user_info;
use crate::{
    database::{
        models::{Food, NewFood, Shop, ShopMenu},
        mysql::get_mysql_connection,
        schema::{food, shop, shop_menu, shop_menu_link_food},
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
pub fn get_menu_food(request: &Request) -> juri::Result<Response> {
    let user = get_user_info(request.header("token"))?;
    let menu_id: i32 = request
        .query("menu_id")
        .ok_or_status_4001()?
        .parse()
        .ok_or_status_4001()?;

    let conn = &mut get_mysql_connection();
    let shop: Shop = shop::table
        .filter(shop::user_id.eq(user.id))
        .first(conn)
        .ok_or_status_4001()?;

    let shop_menu: ShopMenu = shop_menu::table
        .filter(shop_menu::shop_id.eq(shop.id))
        .filter(shop_menu::id.eq(menu_id))
        .first(conn)
        .ok_or_status_4001()?;

    let food_vec: Vec<Food> = food::table
        .inner_join(shop_menu_link_food::table)
        .filter(shop_menu_link_food::menu_id.eq(shop_menu.id))
        .select(food::all_columns)
        .load(conn)
        .ok_or_status_4001()?;

    let food_vec_json: Vec<Value> = food_vec
        .iter()
        .map(|food| {
            json!({
                "id": food.id,
                "title": food.title,
                "value": food.value
            })
        })
        .collect();
    let data = json!(food_vec_json);
    Ok(result_response::success_data("获取成功", &data)?)
}

#[handler]
pub fn add_food(request: &Request) -> juri::Result<Response> {
    let user = get_user_info(request.header("token"))?;
    let body_json: Value = request.json_value().ok_or_status_4001()?;
    let title = body_json["title"].as_str().ok_or_status_4001()?;
    let ingredient = body_json["ingredient"]
        .as_str()
        .map_or(String::new(), |v| v.to_string());
    let method = body_json["method"]
        .as_str()
        .map_or(String::new(), |v| v.to_string());
    let food_value = body_json["value"].as_str().ok_or_status_4001()?;
    let mut menu_id_vec: Vec<i32> = vec![];

    for menu_id in body_json["menu_id_vec"]
        .as_array()
        .ok_or_status_4001()?
        .iter()
    {
        menu_id_vec.push(menu_id.as_i64().ok_or_status_4001()? as i32);
    }

    let conn = &mut get_mysql_connection();

    let new_food = NewFood {
        user_id: user.id,
        title: title.to_string(),
        ingredient,
        method,
        value: food_value.to_string(),
    };
    diesel::insert_into(food::table)
        .values(&new_food)
        .execute(conn)
        .ok_or_status_4001()?;
    let food: Food = food::table
        .order(food::id.desc())
        .first(conn)
        .ok_or_status_4001()?;

    for menu_id in menu_id_vec {
        let new_link = NewShopMenuLinkFood {
            food_id: food.id,
            menu_id,
        };
        diesel::insert_into(shop_menu_link_food::table)
            .values(&new_link)
            .execute(conn)
            .ok_or_status_4001()?;
    }

    Ok(result_response::status_2000("添加成功")?)
}
