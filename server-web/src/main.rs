mod router;
mod types;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket_sync_db_pools::database;
use router::{
    api::{login, register},
    hello,
};

#[database("sqlite_rss")]
pub struct DbConn(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/api", routes![login, register])
        .attach(DbConn::fairing())
}
