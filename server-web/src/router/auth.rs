use crate::{
    types::{
        auth::AccountInfo,
        ResMsg,
        SuccessStatus,
        
    },
    DbConn,
    error::RssError,
    database,
    utility::Jwt,
};
use rocket::serde::json::{serde_json::json, Json, Value};

#[post("/register", format = "json", data = "<info>")]
pub async fn register(info: Json<AccountInfo>, conn: DbConn) ->  Result<Value, RssError>{
    let AccountInfo { username, passwd } = info.into_inner();

    conn.run({
        |con| {
            database::auth::register(con, username, passwd)?;
            Ok(json!(ResMsg{ status: SuccessStatus::REGISTER, msg: String::from("success"), ..Default::default()}))
        }
    })
    .await
}


#[post("/login", format = "json", data = "<info>")]
pub async fn login(info: Json<AccountInfo>, conn: DbConn) -> Result<Value, RssError> {
    let info = info.into_inner();

    conn.run(move |con| {
        database::auth::login(con, info.username.clone(), info.passwd.clone())?;
        let token = Jwt::form(info.username, info.passwd)?;

        Ok(json!(ResMsg{ status: SuccessStatus::LOGIN, msg: String::from("success"), token, ..Default::default()}))               
    }).await
}

