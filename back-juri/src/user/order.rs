use crate::{
    database::{
        models::{NewOrder, NewOrderFood, Order, OrderFood},
        mysql::get_mysql_connection,
        schema::{order, order_food},
    },
    utils::{
        ext::{AsValueExt, OptionResponseExt, ResulResponseExt},
        order::OrderStatus,
        result_response,
        user::get_user_info,
    },
};
use chrono::Local;
use diesel::prelude::*;
use juri::{handler, json::JsonRequestExt, Request, Response};
use serde_json::{json, Value};

#[handler]
pub fn add_order(request: &Request) -> juri::Result<Response> {
    let user = get_user_info(request.header("token"))?;
    let body_json = request.json_value().ok_or_status_4001()?;
    let shop_id = body_json["shop_id"].as_i32().ok_or_status_4001()?;
    let food_vec = body_json["food_vec"].as_array().ok_or_status_4001()?;

    let conn = &mut get_mysql_connection();

    let new_order = NewOrder {
        user_id: user.id,
        shop_id,
        status: OrderStatus::WAIT.as_str().to_string(),
        reason: "".to_string(),
        remark: "".to_string(),
        reserve_start_time: Local::now().naive_local(),
        reserve_end_time: Local::now().naive_local(),
    };

    diesel::insert_into(order::table)
        .values(&new_order)
        .execute(conn)
        .ok_or_status_4001()?;

    let new_order: Order = order::table
        .order(order::id.desc())
        .first(conn)
        .ok_or_status_4001()?;

    for food_data in food_vec.iter() {
        let food_id = food_data["id"].as_i32().ok_or_status_4001()?;
        let food_title = food_data["title"].as_str().ok_or_status_4001()?.to_string();
        let food_value = food_data["value"].as_str().ok_or_status_4001()?.to_string();
        let count = food_data["count"].as_i32().ok_or_status_4001()?;
        let new_order_food = NewOrderFood {
            order_id: new_order.id,
            food_id,
            food_title,
            food_value,
            count,
        };

        diesel::insert_into(order_food::table)
            .values(&new_order_food)
            .execute(conn)
            .ok_or_status_4001()?;
    }

    Ok(result_response::status_2000("添加成功")?)
}

#[handler]
pub fn get_order(request: &Request) -> juri::Result<Response> {
    let user = get_user_info(request.header("token"))?;
    let conn = &mut get_mysql_connection();
    let order_vec: Vec<Order> = order::table
        .filter(order::user_id.eq(user.id))
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

#[handler]
pub fn get_order_detail(request: &Request) -> juri::Result<Response> {
    let _user = get_user_info(request.header("token"))?;
    let order_id: i32 = request
        .query("order_id")
        .ok_or_status_4001()?
        .parse()
        .ok_or_status_4001()?;

    let conn = &mut get_mysql_connection();
    let order_data: Order = order::table
        // .filter(order::user_id.eq(user.id))
        .filter(order::id.eq(order_id))
        .first(conn)
        .ok_or_status_4001()?;

    let order_food_vec: Vec<OrderFood> = order_food::table
        .filter(order_food::order_id.eq(order_data.id))
        .load(conn)
        .ok_or_status_4001()?;

    let order_food_vec_json: Vec<Value> = order_food_vec
        .iter()
        .map(|food| {
            json!({
                "id": food.id,
                "title": food.food_title,
                "value": food.food_value,
                "count": food.count
            })
        })
        .collect();
    let data = json!({
        "id": order_data.id,
        "user_id": order_data.user_id,
        "shop_id": order_data.shop_id,
        "status": order_data.status,
        "create_time": order_data.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        "food_vec": order_food_vec_json
    });
    Ok(result_response::success_data("获取成功", &data)?)
}
