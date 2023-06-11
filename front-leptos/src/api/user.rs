use super::net::{get, post};
use crate::model::{OrderModel, SimpleOrderModel};
use leptos::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct NewOrder {
    pub shop_id: i32,
    pub food_vec: Vec<NewOrderFood>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct NewOrderFood {
    pub id: i32,
    pub title: String,
    pub value: String,
    pub count: i32,
}

pub fn new_shop_order_api(data: NewOrder, callback: impl Fn(Result<(), String>) -> () + 'static) {
    spawn_local(async move {
        let res = post::<()>("/api/user/order/add", &data).await;
        if res.code != 2000 {
            callback(Err(res.reason));
            return;
        }
        callback(Ok(()));
    });
}

pub fn get_order_api(callback: impl Fn(Result<Vec<SimpleOrderModel>, String>) -> () + 'static) {
    spawn_local(async move {
        let res = get::<_, Vec<_>, String>("/api/user/order", None).await;
        if res.code != 2000 {
            callback(Err(res.reason));
            return;
        }
        let Some(data) = res.data else {
            callback(Err(String::from("Return SimpleOrderModel error")));
            return;
        };
        callback(Ok(data));
    });
}

pub fn get_order_detail_api(order_id: i32, callback: impl Fn(Result<OrderModel, String>) -> () + 'static) {
    spawn_local(async move {
        let query = [("order_id", order_id.to_string())].to_vec();
        let res = get::<_, Vec<_>, String>("/api/user/order/detail", Some(query)).await;
        if res.code != 2000 {
            callback(Err(res.reason));
            return;
        }
        let Some(data) = res.data else {
            callback(Err(String::from("Return OrderModel error")));
            return;
        };
        callback(Ok(data));
    });
}
