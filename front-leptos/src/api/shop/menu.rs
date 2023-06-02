use crate::model::ShopMenuModel;
use super::super::net::post;
use leptos::spawn_local;
use serde::{Deserialize, Serialize};

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