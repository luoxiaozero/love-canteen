use super::{result_response, log};
use core::fmt::Debug;

pub trait OptionResponseExt<T> {
    fn ok_or_status_4001(self) -> juri::Result<T>;
}

impl<T> OptionResponseExt<T> for Option<T> {
    fn ok_or_status_4001(self) -> juri::Result<T> {
        match self {
            Some(v) => Ok(v),
            None => Err(result_response::status_4001()?)?,
        }
    }
}

pub trait ResulResponseExt<T, E> {
    fn ok_or_status_4001(self) -> juri::Result<T>;
    fn ok_or_status_4030(self) -> juri::Result<T>;
    fn ok_or_status_5000(self) -> juri::Result<T>;
    fn ok_or_status_reason(self, code: u16, reason: &str) -> juri::Result<T>;
}

impl<T, E: Debug> ResulResponseExt<T, E> for Result<T, E> {
    fn ok_or_status_4001(self) -> juri::Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => {
                log::info(&format!("Result Error: {:#?}", err));
                Err(result_response::status_4001()?)?
            },
        }
    }
    fn ok_or_status_4030(self) -> juri::Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(_) => Err(result_response::status_4030()?)?,
        }
    }
    fn ok_or_status_5000(self) -> juri::Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(_) => Err(result_response::status_5000()?)?,
        }
    }
    fn ok_or_status_reason(self, code: u16, reason: &str) -> juri::Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(_) => Err(result_response::status_reason(code, reason)?)?,
        }
    }
}

pub trait AsValueExt {
    fn as_i32(&self) -> Option<i32>;
}

impl AsValueExt for serde_json::Value {
    fn as_i32(&self) -> Option<i32> {
        if let Some(value) = self.as_i64() {
            value.to_string().parse::<i32>().ok()
        } else {
            None
        }
    }
}
