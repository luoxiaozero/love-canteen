use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ResultResponse<T> {
    pub code: u16,
    pub reason: String,
    pub data: Option<T>,
}

async fn post_basic<T>(
    url: &str,
    data: &impl Serialize,
) -> Result<ResultResponse<T>, gloo_net::Error>
where
    T: DeserializeOwned,
{
    let request = Request::post(url).json(data)?;
    let response = request.send().await?;

    let status = response.status();
    if status != 200 {
        return Err(gloo_net::Error::GlooError(response.status_text()));
    }

    let data = response.json::<ResultResponse<T>>().await?;
    Ok(data)
}

pub async fn post<T>(url: &str, data: &impl Serialize) -> ResultResponse<T>
where
    T: DeserializeOwned,
{
    match post_basic::<T>(url, data).await {
        Ok(response) => response,
        Err(err) => match err {
            gloo_net::Error::JsError(err) => ResultResponse {
                code: 4000,
                reason: err.to_string(),
                data: None,
            },
            gloo_net::Error::SerdeError(err) => ResultResponse {
                code: 4000,
                reason: err.to_string(),
                data: None,
            },
            gloo_net::Error::GlooError(err) => ResultResponse {
                code: 4000,
                reason: err,
                data: None,
            },
        },
    }
}
