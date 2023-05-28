use super::{log, result_response};
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

pub struct UserInfo {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoJwt {
    pub id: i32,
    pub exp: i64,
}

pub fn get_user_info(token: Option<String>) -> juri::Result<UserInfo> {
    if let Some(token) = token {
        dotenv().ok();
        let login_serret = env::var("LOGIN_SECRET").expect("LOGIN_SECRET must be set");
        if let Ok(user_info_jwt) = decode::<UserInfoJwt>(
            &token,
            &DecodingKey::from_secret(login_serret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            return Ok(UserInfo {
                id: user_info_jwt.claims.id,
            });
        } else {
            log::info("登录失效，jwt 解密失败")
        }
    } else {
        log::info("登录失效，token 为空")
    }
    Err(result_response::status_reason(4010, "登录失效")?)?
}
