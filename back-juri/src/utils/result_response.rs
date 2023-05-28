use juri::{json::JsonResponseExt, Response};
use serde::Serialize;
use serde_json::json;
use super::log;

pub fn status_reason(code: u16, reason: &str) -> juri::Result<Response> {
    let john = json!({
        "code": code,
        "reason": reason,
    });
    log::info(&format!("{} {}", code, reason));
    Ok(Response::json(&john)?)
}

pub fn status_2000(reason: &str) -> juri::Result<Response> {
    status_reason(2000, reason)
}

/// 语义有误
pub fn status_4001() -> juri::Result<Response> {
    status_reason(4001, "语义有误")
}

/// 无权限
pub fn status_4030() -> juri::Result<Response> {
    status_reason(4030, "无权限")
}

/// 服务器错误
pub fn status_5000() -> juri::Result<Response> {
    status_reason(5000, "服务器错误")
}

pub fn success_data<T>(reason: &str, data: &T) -> juri::Result<Response>
where
    T: ?Sized + Serialize,
{
    let john = json!({
        "code": 2000,
        "reason": reason,
        "data": data
    });
    Ok(Response::json(&john)?)
}
