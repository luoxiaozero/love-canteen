use super::super::net::{get, post};
use crate::model::FoodModel;
use leptos::spawn_local;
use serde::{Deserialize, Serialize};

pub fn get_food_vec_api(
    menu_id: i32,
    callback: impl Fn(Result<Vec<FoodModel>, String>) -> () + 'static,
) {
    spawn_local(async move {
        let query = [("menu_id", menu_id.to_string())].to_vec();
        let res = get::<_, Vec<_>, String>("/api/shop/food", Some(query)).await;
        if res.code != 2000 {
            callback(Err(res.reason));
            return;
        }
        let Some(data) = res.data else {
            callback(Err(String::from("Return Vec<FoodModel> error")));
            return;
        };
        callback(Ok(data));
    });
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct NewFood {
    pub title: String,
    pub value: String,
}

pub fn new_food_api(data: NewFood, callback: impl Fn(Result<(), String>) -> () + 'static) {
    spawn_local(async move {
        let res = post::<()>("/api/shop/food/add", &data).await;
        if res.code != 2000 {
            callback(Err(res.reason));
            return;
        }
        callback(Ok(()));
    });
}
