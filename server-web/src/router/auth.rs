use crate::types::database::{login_state, user};
use crate::{
    types::{
        auth::{AccountInfo, LoginStateInfo, UserQuery},
        ResMsg, ResType,
    },
    DbConn,
};
use diesel::{
    prelude::*,
    result::{DatabaseErrorKind::UniqueViolation, Error},
    RunQueryDsl,
};
use rocket::{
    http::Status,
    response::status,
    serde::json::{serde_json::json, Json},
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
pub async fn login(info: Json<AccountInfo>, conn: DbConn) -> ResType {
    let info = info.into_inner();

    conn.run(move |con| {
        let passwd_query_res = user::table
            .filter(user::username.eq(info.username.clone()))
            .select(user::passwd)
            .first::<String>(con);

        match passwd_query_res {
            Err(e) => match e {
                Error::NotFound => {
                    status::Custom(
                        Status::Ok,
                        json!(ResMsg {
                            status: 403,
                            msg: String::from("unregistered"),
                            ..Default::default()
                        }),
                    )
                },
                _ => status::Custom(Status::InternalServerError, json!(())),
            },
            Ok(p) => {
                if p == info.passwd.clone() {
                    // passwd correct
                    let session_data = Uuid::new_v4().to_string();
                    let state_info = LoginStateInfo {
                        username: info.username,
                        session_data: session_data.clone(),
                    };
                    
                    let insert_query_res = diesel::insert_into(login_state::table).values(state_info).execute(con);
                    match insert_query_res {
                        Ok(_) => {
                            
                            status::Custom(
                                Status::Ok,
                                json!(ResMsg {
                                    status: 201,
                                    msg: String::from("success"),
                                    session_data
                                }),
                            )
                        },
                        Err(e) => match e {
                            Error::DatabaseError(UniqueViolation, _) => status::Custom(
                                Status::Ok,
                                json!(ResMsg {
                                    status: 402,
                                    msg: String::from("You have logged in to the account"),
                                    ..Default::default()
                                }),
                            ),
                            _ => status::Custom(Status::InternalServerError, json!(())),
                        },
                    }
                } else {
                    // passwd error
                    status::Custom(
                        Status::Ok,
                        json!(ResMsg {
                            status: 401,
                            msg: String::from("fail"),
                            ..Default::default()
                        }),
                    )
                }
            }
        }
    })
    .await
}
