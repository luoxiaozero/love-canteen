use crate::store::Token;
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
    let mut request = Request::post(url);
    if let Some(token) = Token::get() {
        request = request.header("token", &token);
    }
    request = request.json(data)?;
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

async fn get_basic<'a, T, P, V>(
    url: &str,
    params: Option<P>,
) -> Result<ResultResponse<T>, gloo_net::Error>
where
    T: DeserializeOwned,
    P: IntoIterator<Item = (&'a str, V)>,
    V: AsRef<str>,
{
    let mut request = Request::get(url);
    if let Some(token) = Token::get() {
        request = request.header("token", &token);
    }
    if let Some(params) = params {
        request = request.query(params);
    }
    let response = request.send().await?;

    let status = response.status();
    if status != 200 {
        return Err(gloo_net::Error::GlooError(response.status_text()));
    }

    let data = response.json::<ResultResponse<T>>().await?;
    Ok(data)
}

pub async fn get<'a, T, P, V>(url: &str, params: Option<P>) -> ResultResponse<T>
where
    T: DeserializeOwned,
    P: IntoIterator<Item = (&'a str, V)>,
    V: AsRef<str>,
{
    match get_basic::<T, P, V>(url, params).await {
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
