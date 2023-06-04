mod menu;
mod food;

use juri::Router;

pub fn router() -> Router {
    let mut router = Router::new();
    router.at("/shop/menu").get(menu::get_menu);
    router.at("/shop/menu/add").post(menu::add_menu);
    router.at("/shop/menu/food").get(food::get_menu_food);
    router.at("/shop/food/add").post(food::add_food);
    router
}