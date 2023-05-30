use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ResultResponse<T> {
    pub code: u16,
    pub reason: String,
    pub data: Option<T>,
}

async fn post_basic<T>(
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
        Ok(response) => Ok(response),
        Err(err) => Err(surf::Error::from_str(status, err.to_string())),
    }
}

pub async fn post<T>(uri: impl AsRef<str>, data: &impl Serialize) -> ResultResponse<T>
where
    T: DeserializeOwned,
{
    match post_basic::<T>(uri, data).await {
        Ok(response) => response,
        Err(err) => ResultResponse {
            code: err.status().into(),
            reason: err.to_string(),
            data: None,
        },
    }
}
