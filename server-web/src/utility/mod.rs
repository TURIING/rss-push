use crate::types::database::login_state;
use crate::error::{ RssError, AuthErrorKind::UserNotExist};

use anyhow::Result;
use diesel::SqliteConnection;
use diesel::{QueryDsl, RunQueryDsl, result::Error::NotFound, prelude::*};

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