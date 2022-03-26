use crate::types::{
    database::login_state,
    auth::Claims,
};
use crate::constant::{JWTSECRET, JWTALG};
use crate::error::{ RssError, AuthErrorKind::UserNotExist};

use diesel::{SqliteConnection,QueryDsl, RunQueryDsl, result::Error::NotFound, prelude::*};
use jsonwebtoken::{ EncodingKey, DecodingKey, Validation };

pub fn get_username_by_session(token: String, con: &mut SqliteConnection) -> Result<String, RssError> {
    
    let username_query = login_state::table
        .filter(login_state::token.eq(token))
        .select(login_state::username)
        .first::<String>(con);
    match username_query {
        Ok(username) => {
            Ok(username)
        },
        Err(e) => {
            match e {
                NotFound => Err(RssError::AuthError(UserNotExist)),
                _ => Err(RssError::UnknownError(e.to_string()))
            }
        }
    }
}

pub struct Jwt;
impl Jwt {
    // return jwt token
    pub fn form(username: String, passwd: String) -> Result<String, RssError> {
        let claims = Claims { username, passwd, ..Default::default() };
        //let header = jsonwebtoken::Header { alg: JWTALG, ..Default::default() };
        let header = jsonwebtoken::Header::new(JWTALG);

        match jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(JWTSECRET)) {
            Ok(token) => {
                Ok(token)
            },
            Err(e) => Err(RssError::JwtInvalidError(e))
        }
    }

    // return username
    pub fn validate(token: String) -> Result<String, RssError> {
        match jsonwebtoken::decode::<Claims>(
            &token, 
            &DecodingKey::from_secret(JWTSECRET), 
            &Validation::new(JWTALG)
        ) {
            Ok(token_data) => Ok(token_data.claims.username),
            Err(e) => Err(RssError::JwtInvalidError(e)),
        }
    }
}