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
