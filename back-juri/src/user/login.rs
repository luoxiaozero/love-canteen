use crate::{
    database::{models::User, mysql::get_mysql_connection, schema::user},
    utils::{
        ext::{OptionResponseExt, ResulResponseExt},
        result_response,
        user::UserInfoJwt,
    },
};
use chrono::{Duration, Local};
use diesel::prelude::*;
use dotenv::dotenv;
use juri::{handler, json::JsonRequestExt, Request, Response};
use serde_json::json;
use std::env;

#[handler]
pub fn login(request: &Request) -> juri::Result<Response> {
    let body_json = request.json_value().ok_or_status_4001()?;
    let account = body_json["account"].as_str().ok_or_status_4001()?;
    let password = body_json["password"].as_str().ok_or_status_4001()?;

    let conn = &mut get_mysql_connection();

    let login_user: User = user::table
        .filter(user::account.eq(account))
        .first::<User>(conn)
        .ok_or_status_4001()?;
    if !bcrypt::verify(password, &login_user.password).unwrap() {
        return Ok(result_response::status_reason(4001, "账号或密码错误")?);
    }

    let expiration_time = (Local::now() + Duration::days(30)).timestamp_millis();

    dotenv().ok();
    let login_serret = env::var("LOGIN_SECRET").expect("LOGIN_SECRET must be set");

    let claims = UserInfoJwt {
        id: login_user.id,
        exp: expiration_time,
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(login_serret.as_ref()),
    )
    .unwrap();

    let data = json!({ "token": token });
    Ok(result_response::success_data("登录成功", &data)?)
}

#[cfg(test)]
mod test {

    #[test]
    fn hash() {
        let password = "love";
        let password = bcrypt::hash(password, 10).unwrap();
        println!("{password}");
    }
}
