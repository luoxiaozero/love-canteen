mod login;
mod order;

use juri::Router;

pub fn router() -> Router {
    let mut router = Router::new();
    router.at("/user/login").post(login::login);
    router.at("/user/order").get(order::get_order);
    router.at("/user/order/add").post(order::add_order);
    router.at("/user/order/detail").get(order::get_order_detail);
    router
}
