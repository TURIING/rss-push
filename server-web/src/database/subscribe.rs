
use diesel::{SqliteConnection, QueryDsl ,RunQueryDsl, prelude::*};
use crate::error::RssError;
//use crate::types::task::SubscribeQuery;
use crate::types::schema::subscribe;

#[derive(Debug, Queryable, Insertable, AsChangeset, Default)]
#[table_name = "subscribe"]
pub struct Subscribe {
    pub id: Option<i32>,
    pub username: String,
    pub crate_id: String
}
impl Subscribe {
    pub fn insert(self ,con: &SqliteConnection) -> Result<(), RssError> {
    
        diesel::insert_into(subscribe::table).values(self).execute(con)?;
        Ok(())

    }

    pub fn delete(username: String, crate_id: String, con: &SqliteConnection) -> Result<(), RssError> {
        diesel::delete(subscribe::table.filter(subscribe::crate_id.eq(crate_id).and(subscribe::username.eq(username)))).execute(con)?;
        Ok(())
    }

    pub fn get_records_by_username(
        con: &SqliteConnection,
        username: String
    ) -> Result<Option<Vec<String>>, RssError> {
        
        let records = subscribe::table
            .filter(subscribe::username.eq(username))
            .select(subscribe::crate_id)
            .load::<String>(con)?;
        if records.len() != 0 {
            Ok(Some(records))
        } else {
            Ok(None)
        }
        
    }
}