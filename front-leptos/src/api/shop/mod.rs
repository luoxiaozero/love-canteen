mod food;
mod menu;

pub use food::*;
pub use menu::*;

use super::net::get;
use crate::model::ShopModel;
use leptos::spawn_local;

pub fn get_shop_vec_api(callback: impl Fn(Result<Vec<ShopModel>, String>) -> () + 'static) {
    spawn_local(async move {
        let res = get::<_, Vec<_>, String>("/api/shop", None).await;
        if res.code != 2000 {
            callback(Err(res.reason));
            return;
        }
        let Some(data) = res.data else {
            callback(Err(String::from("Return Vec<ShopModel> error")));
            return;
        };
        callback(Ok(data));
    });
}

pub fn is_self_shop_api(shop_id: i32, callback: impl Fn(Result<bool, String>) -> () + 'static) {
    spawn_local(async move {
        let query = [("shop_id", shop_id.to_string())].to_vec();
        let res = get::<_, Vec<_>, String>("/api/shop/self", Some(query)).await;
        if res.code != 2000 {
            callback(Err(res.reason));
            return;
        }
        let Some(data) = res.data else {
            callback(Err(String::from("Return Self Shop error")));
            return;
        };
        callback(Ok(data));
    });
}
