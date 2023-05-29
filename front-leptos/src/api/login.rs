use leptos::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct LoginData {
    account: String,
    password: String,
}

pub fn login_api(cx: Scope, data: LoginData) {
    let (data, _) = create_signal(cx, data);
    let login = create_resource(
        cx,
        move || data.get(),
        |data| async move {
            post::<()>("/api/user/login", &data).await;
        },
    );
}

#[derive(Deserialize)]
struct ResultResponse<T> {
    pub code: u16,
    pub reason: String,
    pub data: Option<T>,
}

async fn post<T>(
    uri: impl AsRef<str>,
    data: &impl Serialize,
) -> Result<ResultResponse<T>, surf::Error>
where
    T: DeserializeOwned,
{
    let mut resource = surf::post(uri).body_json(&data)?.await?;
    let status = resource.status();
    if status != surf::StatusCode::Ok {
        return Err(surf::Error::from_str(status, status.canonical_reason()));
    }
    match resource.body_json::<ResultResponse<T>>().await {
        Ok(data) => {
            if data.code == 2000 {
                Ok(data)
            } else {
                Err(surf::Error::from_str(status, data.reason))
            }
        },
        Err(err) => Err(surf::Error::from_str(status, err.to_string())),
    }
}
