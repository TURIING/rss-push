use crate::error::RssError;
use crate::types::{
    database::crates,
    task::{ CratesQuery, CrateInfo},
};
use rocket::serde::json::serde_json;
use diesel::{SqliteConnection, QueryDsl, RunQueryDsl, prelude::*};

pub fn exist_or_insert(
    con: &SqliteConnection,
    url_uuid: String,
    crate_info: CrateInfo
) -> Result<(), RssError> {

    let crates_info = serde_json::to_string(&crate_info).unwrap();
    // Check whether this record exists in the Crates table
    crates::table
    .filter(crates::crate_id.eq(url_uuid.clone()))
    .select(crates::crate_id)
    .first::<String>(con)
    .or_else(|e| {
        match e {
            NotFound => {
                // if does not exist, insert it.
                let insert_info = CratesQuery {
                    crate_id: url_uuid.clone(),
                    crate_type: String::from("rss"),
                    info: crates_info,
                };
                diesel::insert_into(crates::table).values(insert_info).execute(con)?;
                Ok(String::new())
            },
            _ => return Err(e),
        }
    })?;
    Ok(())
}

pub fn get_info_by_id(con: &SqliteConnection, crate_id: String) -> Result<String, RssError> {
    let info = crates::table
                    .filter(crates::crate_id.eq(crate_id))
                    .select(crates::info)
                    .first::<String>(con)?;
    Ok(info)
}