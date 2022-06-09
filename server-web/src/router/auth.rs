use crate::{types::{ResMsg, SuccessStatus}, DbConn, error::RssError, utility::Jwt};
use rocket::serde::json::{serde_json::json, Json, Value};
use crate::database::auth::User;

#[post("/register", format = "json", data = "<info>")]
pub async fn register(info: Json<User>, conn: DbConn) ->  Result<Value, RssError>{
    let user = info.into_inner();

    conn.run(move |con| {
            user.register(con)?;
            Ok(json!(
                    ResMsg{ 
                        status: SuccessStatus::REGISTER, 
                        msg: String::from("success"), 
                        ..Default::default()
                    }
            ))
        }
    )
    .await
}

#[post("/login", format = "json", data = "<info>")]
pub async fn login(info: Json<User>, conn: DbConn) -> Result<Value, RssError> {
    let user = info.into_inner();

    conn.run(move |con| {
        user.login(con)?;
        let token = Jwt::form(user.username, user.passwd)?;

        Ok(json!(
            ResMsg{
                status: SuccessStatus::LOGIN,
                msg: String::from("success"), 
                token, ..Default::default()
            }
        ))               
    }).await
}