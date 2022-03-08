mod router;
mod types;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket_sync_db_pools::database;
use router::{
    auth::{login, register},
    rss::rss_info,
    hello,
};
use rocket_cors::{AllowedOrigins};
use rocket::http::Method::{Post, Get};

#[database("sqlite_rss")]
pub struct DbConn(diesel::SqliteConnection);



#[launch]
fn rocket() -> _ {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: vec![Post, Get].into_iter().map(From::from).collect(),
        ..Default::default()
    }.to_cors().unwrap();

    rocket::build()
        .mount("/", routes![hello])
        .mount("/api/auth", routes![login, register])
        .mount("/api/rss", routes![rss_info])
        .attach(DbConn::fairing())
        .attach(cors)
}
