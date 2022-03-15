use crate::{
    types::{
        auth::AccountInfo,
        ResMsg, 
    },
    DbConn,
    error::RssError,
    database
};
use rocket::serde::json::{serde_json::json, Json, Value};


#[post("/register", format = "json", data = "<info>")]
pub async fn register(info: Json<AccountInfo>, conn: DbConn) ->  Result<Value, RssError>{
    let AccountInfo { username, passwd } = info.into_inner();

    conn.run({
        |con| {
            database::auth::register(con, username, passwd)?;
            Ok(json!(ResMsg{ status: 200, msg: String::from("success"), ..Default::default()}))
        }
    })
    .await
}


#[post("/login", format = "json", data = "<info>")]
pub async fn login(info: Json<AccountInfo>, conn: DbConn) -> Result<Value, RssError> {
    let info = info.into_inner();

    conn.run(|con| {
        let token = database::auth::login(con, info.username, info.passwd)?;

        Ok(json!(ResMsg{ status: 201, msg: String::from("success"), token}))
        
       
    }).await
}

