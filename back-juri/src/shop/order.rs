use crate::{
    database::{
        models::{Order, Shop},
        mysql::get_mysql_connection,
        schema::{order, shop},
    },
    utils::{ext::ResulResponseExt, result_response, user::get_user_info},
};
use diesel::prelude::*;
use juri::*;
use serde_json::{json, Value};

#[handler]
pub fn get_shop_order(request: &Request) -> juri::Result<Response> {
    let user = get_user_info(request.header("token"))?;

    let conn = &mut get_mysql_connection();

    let shop_vec: Vec<Shop> = shop::table
        .filter(shop::user_id.eq(user.id))
        .limit(1)
        .load(conn)
        .ok_or_status_4001()?;

    if shop_vec.is_empty() {
        return Ok(result_response::status_2000("获取成功")?);
    }
    let shop = &shop_vec[0];

    let order_vec: Vec<Order> = order::table
        .filter(order::shop_id.eq(shop.id))
        .load(conn)
        .ok_or_status_4001()?;
    let order_vec_json: Vec<Value> = order_vec
        .iter()
        .map(|order| {
            json!({
                "id": order.id,
                "shop_id": order.shop_id,
                "status": order.status,
                "create_time": order.create_time.format("%Y-%m-%d %H:%M:%S").to_string()
            })
        })
        .collect();
    let data = json!(order_vec_json);
    Ok(result_response::success_data("获取成功", &data)?)
}
