use serde::{Deserialize, Serialize};
use leptos::*;

#[derive(Deserialize, Serialize)]
struct LoginData {
    account: String,
    password: String
}

pub fn login_api(cx: Scope, data: LoginData ) {
    let login = create_resource(cx, || (), |_| async move {
    });
}

// async fn post<T>(uri: impl AsRef<str>, data: &impl Serialize) -> Result<T, surf::Error> {
//     let resource = surf::post(uri).body_json(&data)?.await?;
//     if resource.status() != surf::StatusCode::Ok {
//     }
//     Ok(())
// }