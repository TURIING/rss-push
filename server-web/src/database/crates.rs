use crate::error::RssError;
use crate::types::{
    database::crates,
    task::CratesQuery,
};

use diesel::{SqliteConnection, QueryDsl, RunQueryDsl, prelude::*};

pub fn exist_or_insert(
    con: &SqliteConnection, 
    url_uuid: String,    
) -> Result<(), RssError> {

    // Check whether this record exists in the Crates table
    crates::table
    .filter(crates::crates_id.eq(url_uuid.clone()))
    .select(crates::crates_id)
    .first::<String>(con)
    .or_else(|e| {
        match e {
            NotFound => {
                // if does not exist, insert it.
                let crate_info = CratesQuery {
                    crates_id: url_uuid.clone(),
                    crates_type: String::from("rss"),
                    info: String::new()
                };
                diesel::insert_into(crates::table).values(crate_info).execute(con)?;
                Ok(String::new())
            },
            _ => return Err(e),
        }
    })?;
    Ok(())
}