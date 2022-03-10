use crate::{
    DbConn,
    types::database::login_state
};
use crate::error::{ Error, AuthErrorKind::UserNotExist};

use anyhow::Result;
use diesel::SqliteConnection;
use diesel::{QueryDsl, RunQueryDsl, result::Error::NotFound};

pub fn get_username_by_session(session_data: String, con: &mut SqliteConnection) -> Result<String, Error> {

    
    let username_query = login_state::table
        .filter(login_state::session_data.eq(session_data))
        .select(login_state::username)
        .first::<String>(con);
    match username_query {
        Ok(username) => {
            Ok(username)
        },
        Err(e) => {
            match e {
                NotFound => Err(Error::AuthError(UserNotExist)),
                _ => Err(Error::UnknownError(e.to_string()))
            }
        }
    }
    
    
    
}