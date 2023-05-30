use super::net::post;
use leptos::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct LoginData {
    pub account: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResource {
    pub token: String,
}

pub fn login_api(
    data: LoginData,
    callback: impl Fn(Result<LoginResource, String>) -> () + 'static,
) {
    spawn_local(async move {
        let res = post::<LoginResource>("/api/user/login", &data).await;
        if res.code != 2000 {
            callback(Err(res.reason));
            return;
        }
        let Some(data) = res.data else {
            callback(Err(String::from("Return LoginResource error")));
            return;
        };
        callback(Ok(data));
    });
}
