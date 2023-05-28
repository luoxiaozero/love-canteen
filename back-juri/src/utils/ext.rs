use super::result_response;

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

impl<T, E> ResulResponseExt<T, E> for Result<T, E> {
    fn ok_or_status_4001(self) -> juri::Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(_) => Err(result_response::status_4001()?)?,
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
