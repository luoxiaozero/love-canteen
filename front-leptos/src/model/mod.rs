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