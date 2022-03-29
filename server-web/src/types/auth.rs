use crate::{
    types::database::user,
    error::{ RssError, AuthErrorKind::InvalidToken },
    utility::Jwt,
};
use rocket::{
    serde::{Deserialize, Serialize}, 
    request::{ FromRequest, Request, Outcome },
    http::Status,

};
use time::{ OffsetDateTime,Duration };

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

// structure for jwt
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: i64,
    pub username: String,
    pub passwd: String
}
impl Default for Claims {
    fn default() -> Self {
        let timestamp = OffsetDateTime::now_utc() + Duration::days(1);
        Claims {
            iss: String::from("TURIING"), 
            sub: String::from("rss-push"),
            exp: timestamp.unix_timestamp(),
            username: String::new(),
            passwd: String::new()
        }
    }
}

pub struct Token(pub String);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = RssError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get("Authorization").next() {
            Some(data) => {
                let data: Vec<&str> = data.split_whitespace().collect();
                if data.len() != 2 {
                    return Outcome::Failure((Status::BadRequest, RssError::AuthError(InvalidToken)));
                }
                match Jwt::validate(data[1].to_string()) {
                    Ok(username) => Outcome::Success(Token(username)),
                    Err(e) => Outcome::Failure((Status::BadRequest, e)),
                }                
            },
            None => Outcome::Failure((Status::BadRequest, RssError::AuthError(InvalidToken)))
        }
    }
}