use crate::error::RssError;
use crate::types::schema::crates;
use diesel::{SqliteConnection, QueryDsl, RunQueryDsl, prelude::*};

#[derive(Insertable, AsChangeset)]
#[table_name = "crates"]
pub struct Crates {
    pub crate_id: String,
    pub crate_type: i32,
    pub info: String
}

impl Crates {
    pub fn exist_or_insert(&self, con: &SqliteConnection) -> Result<(), RssError> {
    
        // Check whether this record exists in the Crates table
        crates::table
        .filter(crates::crate_id.eq(self.crate_id.clone()))
        .select(crates::crate_id)
        .first::<String>(con)
        .or_else(|e| {
            match e {
                NotFound => {
                    // if does not exist, insert it.
                    diesel::insert_into(crates::table).values(self).execute(con)?;
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
}

