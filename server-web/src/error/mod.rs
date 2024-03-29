use thiserror::Error;
use rocket::{
    response::{ self, Responder, Response}, 
    request::Request,
    serde::json::serde_json::json
};
use self::AuthErrorKind::*;
use crate::types::ResMsg;
use std::{
    option::Option::None,
    io::Cursor

};


#[derive(Debug, Error)]
pub enum RssError {
    #[error("Authentication error.")]
    AuthError(AuthErrorKind),
    #[error("Unknown error.")]
    UnknownError(String),
    #[error("Database error.")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("Network error.")]
    NetWorkError(#[from] reqwest::Error),
    #[error("Rss parse error.")]
    RssParseError(#[from] rss::Error),
    #[error("Jwt invalid.")]
    JwtInvalidError(#[from] jsonwebtoken::errors::Error)
}
impl<'r, 'o: 'r> Responder<'r, 'o> for RssError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'o> {
        
        let msg = match self {
            RssError::AuthError(kind) => {
                match kind {
                    UserNotExist => ResMsg{ status: 404, msg: kind.to_string(), ..Default::default()},
                    PasswdMistake => ResMsg { status: 401, msg: kind.to_string(), ..Default::default()},
                    AlreadyLogged => ResMsg { status: 402, msg: kind.to_string(), ..Default::default()},
                    AlreadyRegister => ResMsg { status: 400, msg: kind.to_string(), ..Default::default()},
                    InvalidToken => ResMsg { status: 403, msg: kind.to_string(), ..Default::default()}
                }
            },
            RssError::UnknownError(e) => ResMsg { status: 500, msg: e.to_string(), ..Default::default()},
            RssError::DatabaseError(e) => ResMsg { status: 501, msg: e.to_string(), ..Default::default()},
            RssError::NetWorkError(e) => ResMsg { status: 502, msg: e.to_string(), ..Default::default()},
            RssError::RssParseError(e) => ResMsg { status: 503, msg: e.to_string(), ..Default::default()},
            RssError::JwtInvalidError(e) => ResMsg { status: 504, msg: e.to_string(), ..Default::default()},
        };

        Response::build()
            .sized_body(None, Cursor::new(json!(msg).to_string()))
            .ok()
    }
}

#[derive(Debug, Error)]
pub enum AuthErrorKind {
    #[error("The user does not exist")]
    UserNotExist,
    #[error("The account or password is incorrect")]
    PasswdMistake,
    #[error("You have logged in to the account")]
    AlreadyLogged,
    #[error("This account has been registered")]
    AlreadyRegister,
    #[error("Invalid token")]
    InvalidToken,
}