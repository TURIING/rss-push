use crate::types::database::{login_state, user};
use crate::{
    types::{
        auth::{AccountInfo, LoginStateInfo},
        ResMsg, ResType,
    },
    DbConn,
    error::{ RssError, AuthErrorKind::{ UserNotExist, AlreadyLogged, PasswdMistake}}
};
use diesel::{
    prelude::*,
    result::{DatabaseErrorKind::UniqueViolation, Error},
    RunQueryDsl,
};
use rocket::{
    http::{ Status, CookieJar, Cookie},
    response::status,
    serde::json::{serde_json::json, Json, Value},
};
use uuid::Uuid;

// register account
// 200 success
// 400 already register
#[post("/register", format = "json", data = "<info>")]
pub async fn register(info: Json<AccountInfo>, conn: DbConn) -> ResType {
    let info = info.into_inner();

    conn.run({
        |con| {
            let res = diesel::insert_into(user::table).values(info).execute(con);

            match res {
                Ok(_) => {
                    status::Custom(
                        Status::Ok,
                        json!(
                            ResMsg {
                                status: 200,
                                msg: String::from("success"),
                                ..Default::default()
                            }
                        ),
                    )
                }
                ,
                Err(e) => {
                    match e {
                        Error::DatabaseError(UniqueViolation, _) => {
                            status::Custom(
                                Status::Ok,
                                json!(ResMsg {
                                    status: 400,
                                    msg: String::from("Has already been registered"),
                                    ..Default::default()
                                }),
                            )
                        },
                        _ => status::Custom(Status::InternalServerError, json!(())),
                    }
                }
            }
        }
    })
    .await
}

// login the account
// 201 success
// 401 passwd error
// 402 already login
// 403 unregistered
#[post("/login", format = "json", data = "<info>")]
pub async fn login(info: Json<AccountInfo>, conn: DbConn, cookie: &CookieJar<'_>) -> Result<Value, RssError> {
    let info = info.into_inner();

    conn.run(move |con| {
        let passwd_query_res = user::table
            .filter(user::username.eq(info.username.clone()))
            .select(user::passwd)
            .first::<String>(con);

        match passwd_query_res {
            Err(e) => match e {
                Error::NotFound => { return Err(RssError::AuthError(UserNotExist)) },
                _ => return Err(e),
            },
            Ok(p) => {
                if p == info.passwd.clone() {
                    // passwd correct
                    let token = Uuid::new_v4().to_string();
                    let state_info = LoginStateInfo {
                        username: info.username,
                        session_data: token.clone(),
                    };
                    
                    let insert_query = diesel::insert_into(login_state::table).values(state_info).execute(con);
                    match insert_query {
                        Ok(_) => {
                            cookie.add(Cookie::new("token", token));
                            json!(ResMsg { status: 201, msg: String::from("success"), ..Default::default()})
                        },
                        Err(e) => match e {
                            Error::DatabaseError(UniqueViolation, _) => return Err(RssError::AuthError(AlreadyLogged)),
                            _ => return Err(e),
                        },
                    }
                } else {
                    // passwd error
                    return Err(RssError::AuthError(PasswdMistake))
                }
            }
        }
    })
    .await
}