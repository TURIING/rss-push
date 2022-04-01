use diesel::{SqliteConnection, QueryDsl ,RunQueryDsl, prelude::*};
use crate::error::RssError;
use crate::types::database::MessageContentQuery;
use crate::types::schema::message_content;

pub fn insert(
    message_id: String,
    send_time: Option<String>,
    content: String,
    con: &SqliteConnection
) -> Result<(), RssError> {

    let send_time = send_time.unwrap();
    let query = MessageContentQuery { id: None, message_id, send_time, content };
    diesel::insert_into(message_content::table).values(query).execute(con)?;

    Ok(())
}
