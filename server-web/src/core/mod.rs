use diesel::prelude::*;

use crate::constant::DATABASEURL;
use crate::error::RssError;

pub async fn check_update() -> Result<(), RssError> {
    let con = SqliteConnection::establish(DATABASEURL)
        .expect("The database URL could not be found");
    
    if let Some(query) = task::get_records_by_id(&con, 0, 100)? {
        println!("{:?}", query);
    } else {
        println!("none");
    }

    Ok(())
}