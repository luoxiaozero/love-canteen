use gloo_storage::{LocalStorage, Storage};

const DEFAULT_SHOP_ID: &str = "default_shop_id";
pub struct DefaultShopId;

impl DefaultShopId {
    pub fn get() -> Option<i32> {
        let Ok(shop_id) = LocalStorage::get(DEFAULT_SHOP_ID) else {
            return None;
        };

        shop_id
    }

    pub fn set(shop_id: i32) {
        if shop_id == 0 {
            LocalStorage::delete(DEFAULT_SHOP_ID)
        } else {
            _ = LocalStorage::set(DEFAULT_SHOP_ID, shop_id);
        }
    }
}