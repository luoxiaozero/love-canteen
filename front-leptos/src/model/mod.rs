use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShopMenuModel {
    pub id: u32,
    pub title: String,
}
