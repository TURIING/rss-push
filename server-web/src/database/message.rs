use diesel::{SqliteConnection, QueryDsl ,RunQueryDsl, prelude::*};
use crate::error::RssError;
use crate::types::database::MessageQuery;
use crate::types::schema::message;

pub fn insert(
    message_id: String,
    crate_id: String,
    receiver: String,
    con: &SqliteConnection
) -> Result<(), RssError> {
    
    let query = MessageQuery { id: None, message_id, crate_id, receiver, check_status: String::from("false") };
    diesel::insert_into(message::table).values(query).execute(con)?;

    Ok(())
}