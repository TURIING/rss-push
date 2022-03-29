
use diesel::{SqliteConnection, QueryDsl ,RunQueryDsl, prelude::*};
use crate::error::RssError;
//use crate::types::task::SubscribeQuery;
use crate::types::schema::subscribe;
use crate::types::database::SubscribeQuery;

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

pub fn insert(
    con: &SqliteConnection, 
    username: String, 
    crate_id: String
) -> Result<(), RssError> {

    let info = SubscribeQuery { id: None, username, crate_id };
    diesel::insert_into(subscribe::table).values(info).execute(con)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use diesel::prelude::*;
    use crate::database::subscribe::get_records_by_id;
    use crate::constant::DATABASEURL;

    #[test]
    fn get_records_by_id_work() {
        let con = SqliteConnection::establish(DATABASEURL)
            .expect("The database URL could not be found");
        println!("{:?}", get_records_by_id(&con, String::from("turiing")));
    }
}