use crate::{
    error::{ 
        RssError, 
        AuthErrorKind::{PasswdMistake, UserNotExist, AlreadyRegister}
    },
    types::schema::user
};
use diesel::SqliteConnection;
use diesel::{
    prelude::*,
    result::{DatabaseErrorKind::UniqueViolation, Error},
    RunQueryDsl,
};
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, AsChangeset, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "user"]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub username: String,
    pub passwd: String,
}

impl User {
    pub fn login(&self, con: &SqliteConnection) -> Result<(), RssError> {
    
         let passwd_query = user::table
                .filter(user::username.eq(self.username.clone()))
                .select(user::passwd)
                .first::<String>(con);
    
        match passwd_query {
            Err(e) => match e {
                Error::NotFound => return Err(RssError::AuthError(UserNotExist)),
                _ => return Err(RssError::DatabaseError(e)),
            },
            Ok(p) => {
                if p == self.passwd {
                    // passwd correct
                    Ok(())
                } else {
                    // passwd error
                    Err(RssError::AuthError(PasswdMistake))
                }
            }
        }
    }

    pub fn register(&self, con: &SqliteConnection) -> Result<(), RssError> {
    
        diesel::insert_into(user::table)
            .values(self)
            .execute(con)
            .or_else(|e| {
                match e {
                    Error::DatabaseError(UniqueViolation, _) => Err(RssError::AuthError(AlreadyRegister)),
                    _ => Err(RssError::DatabaseError(e)),
                }
            })?;
        Ok(())
    }
}




