mod menu;

use juri::Router;

pub fn router() -> Router {
    let mut router = Router::new();
    router.at("/shop/menu").get(menu::get_menu);
    router.at("/shop/menu/add").post(menu::add_menu);
    router
}