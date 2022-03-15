use crate::{
    types::database::{login_state, user},
    error::RssError,
    database::auth::validate_token,
    DbConn,
};
use rocket::{serde::{Deserialize, Serialize}, request::{ FromRequest, self}, Request, outcome::IntoOutcome};

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "user"]
pub struct AccountInfo {
    pub username: String,
    pub passwd: String,
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "user"]
pub struct UserQuery {
    pub id: Option<i32>,
    pub username: String,
    pub passwd: String,
}


#[derive(Insertable, AsChangeset)]
#[table_name = "login_state"]
pub struct LoginStateInfo {
    pub username: String,
    pub token: String,
}

pub struct Token(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = RssError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let conn = req.guard::<DbConn>().await.succeeded().unwrap();
        conn.run(|con| {
            let t = req.cookies()
            .get("token")
            .and_then(|ctx| {
                let token = ctx.value().to_string();
                validate_token(con, token.clone()).unwrap();
                
                Some(token)
            })
            .map(Token)
            .unwrap();
        request::Outcome::Success(t)
        }).await
    }
}
