
use rocket::{
    serde::json::{Json, serde_json::json},
    response::status,
    http::Status
};
use crate::{
    DbConn,
    types::{ ResType, ResMsg, auth::{ AccountInfo, UserQuery, LoginStateInfo} }
};
use crate::types::database::{ User, login_state};
use diesel::{
    RunQueryDsl, 
    prelude::*,
    result::{ Error, DatabaseErrorKind::UniqueViolation }
};

// register account
// 200 success
// 400 already register
#[post("/register", format = "json", data = "<info>")]
pub async fn register(info: Json<AccountInfo>, conn: DbConn) ->  ResType {
    let info = info.into_inner();

    conn.run({
        |con| {

            let res = diesel::insert_into(User::table).values(info).execute(con);

            match res {
                Ok(_) => status::Custom(Status::Ok, json!(ResMsg{status: 200, msg: String::from("success")})),
                Err(e) => {
                    match e {
                        Error::DatabaseError(UniqueViolation, _) => status::Custom(
                            Status::Ok, 
                            json!(ResMsg{status: 400, msg: String::from("Has already been registered")})
                        ),
                        _ => status::Custom(
                            Status::InternalServerError, 
                            json!(())
                        )
                    }
                }
            }
        }
    }).await
}
// login the account
// 201 success
// 401 passwd error
// 402 already login
// 403 unregistered
#[post("/login", format = "json", data = "<info>")]
pub async fn login(info: Json<AccountInfo>, conn: DbConn) -> ResType {
    let info = info.into_inner();

    conn.run(
        move |con| {
            
            let passwd_query_res = User::table.filter(User::username.eq(info.username.clone())).select(User::passwd).first::<String>(con);
            
            match passwd_query_res {
                Err(e) => match e {
                    Error::NotFound => status::Custom(
                        Status::Ok,
                        json!(ResMsg{ status: 403, msg: String::from("unregistered")})
                    ),
                    _ => status::Custom(
                        Status::InternalServerError,
                        json!(())
                    )
                },
                Ok(p) => {
                    if p == info.passwd.clone() {
                        // passwd correct
                        let state_info = LoginStateInfo {
                            username: info.username,
                            session_data: String::from("123456")
                        };
                        let insert_query_res = diesel::insert_into(login_state::table).values(state_info).execute(con);
                        match insert_query_res {
                            Ok(_) => status::Custom(
                                Status::Ok,
                                json!(ResMsg{ status: 201, msg: String::from("success")})
                            ),
                            Err(e) => match e {
                                Error::DatabaseError(UniqueViolation, _) => status::Custom(
                                    Status::Ok,
                                    json!(ResMsg{ status: 402, msg: String::from("You have logged in to the account")})
                                ),
                                _ => status::Custom(
                                    Status::InternalServerError, 
                                    json!(())
                                )
                            }
                        }
                        
                        
                    } else {
                        // passwd error
                        status::Custom(
                            Status::Ok,
                            json!(ResMsg{ status: 401, msg: String::from("fail")})
                        )
                    }
                }
            }
        }
    ).await
}
