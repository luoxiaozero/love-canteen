use super::super::net::{get, post};
use crate::model::ShopMenuModel;
use leptos::spawn_local;
use serde::{Deserialize, Serialize};

pub fn get_shop_menu_api(shop_id: i32, callback: impl Fn(Result<Vec<ShopMenuModel>, String>) -> () + 'static) {
    spawn_local(async move {
        let query = [("shop_id", shop_id.to_string())].to_vec();
        let res = get::<_, Vec<_>, String>("/api/shop/menu", Some(query)).await;
        if res.code != 2000 {
            callback(Err(res.reason));
            return;
        }
        let Some(data) = res.data else {
            callback(Err(String::from("Return ShopMenu error")));
            return;
        };
        callback(Ok(data));
    });
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct NewShopMenu {
    pub title: String,
}

pub fn new_shop_menu_api(
    data: NewShopMenu,
    callback: impl Fn(Result<ShopMenuModel, String>) -> () + 'static,
) {
    spawn_local(async move {
        let res = post("/api/shop/menu/add", &data).await;
        if res.code != 2000 {
            callback(Err(res.reason));
            return;
        }
        let Some(data) = res.data else {
            callback(Err(String::from("Return ShopMenu error")));
            return;
        };
        callback(Ok(data));
    });
}
