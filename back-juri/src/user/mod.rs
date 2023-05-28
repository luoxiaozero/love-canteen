mod login;
use juri::Router;

pub fn router() -> Router {
    let mut router = Router::new();
    router.at("/user/login").post(login::login);
    router
}