use crate::{
    error::{ 
        RssError, 
        AuthErrorKind::{PasswdMistake, UserNotExist, AlreadyRegister}
    },
    types::{ 
        database::user,
        auth::AccountInfo,
    }
};
use diesel::SqliteConnection;
use diesel::{
    prelude::*,
    result::{DatabaseErrorKind::UniqueViolation, Error},
    RunQueryDsl,
};


pub fn login(
    con: &SqliteConnection, 
    username: String,
    passwd: String
) -> Result<(), RssError> {

     let passwd_query = user::table
            .filter(user::username.eq(username.clone()))
            .select(user::passwd)
            .first::<String>(con);

    match passwd_query {
        Err(e) => match e {
            Error::NotFound => return Err(RssError::AuthError(UserNotExist)),
            _ => return Err(RssError::DatabaseError(e)),
        },
        Ok(p) => {
            if p == passwd {
                // passwd correct
                Ok(())
            } else {
                // passwd error
                Err(RssError::AuthError(PasswdMistake))
            }
        }
    }
}

pub fn register(con: &SqliteConnection, username: String, passwd: String) -> Result<(), RssError> {
    
    let info = AccountInfo { username, passwd };

    diesel::insert_into(user::table)
        .values(info)
        .execute(con)
        .or_else(|e| {
            match e {
                Error::DatabaseError(UniqueViolation, _) => Err(RssError::AuthError(AlreadyRegister)),
                _ => Err(RssError::DatabaseError(e)),
            }
        })?;
    Ok(())
}

// pub fn validate_token(con: &SqliteConnection, token: String) -> Result<(), RssError> {

//     login_state::table
//         .filter(login_state::token.eq(token))
//         .select(login_state::username)
//         .first(con)
//         .or_else(|e| {
//             match e {
//                 Error::NotFound => Err(RssError::AuthError(InvalidToken)),
//                 _ => Err(RssError::DatabaseError(e))
//             }
//         })?;
//     Ok(())
// }