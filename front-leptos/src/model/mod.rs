use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShopModel {
    pub id: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShopMenuModel {
    pub id: i32,
    pub title: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FoodModel {
    pub id: i32,
    pub title: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SimpleOrderModel {
    pub id: i32,
    pub shop_id: i32,
    pub status: String,
    pub create_time: String,
}
