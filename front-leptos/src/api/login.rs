use super::net::post;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct LoginData {
    pub account: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResource {
    token: String,
}

pub fn login_api(
    cx: Scope,
    source: impl Fn() -> LoginData + 'static,
) -> Resource<LoginData, Result<LoginResource, String>> {
    create_resource(cx, source, |data| async move {
        let res = post::<LoginResource>("/api/user/login", &data).await;
        if res.code == 2000 {
            res.data
                .map_or(Err(String::from("Return LoginResource error")), |v| Ok(v))
        } else {
            Err(res.reason)
        }
    })
}
