mod router;
mod types;
mod utility;
mod error;
mod database;
mod constant;
mod core;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::{
    http::Method::{Get, Post}, 
    routes, 
    tokio::{self, time::{self, Duration}},
};
use rocket_cors::AllowedOrigins;
use rocket_sync_db_pools::database;
use router::{
    auth::{login, register},
    hello,
    rss::{ info, subscribe}
};

#[database("sqlite_rss")]
pub struct DbConn(diesel::SqliteConnection);

#[rocket::main]
async fn main() {
    tokio::spawn(core::check_update());
    // wait for database
    time::sleep(Duration::new(2, 0)).await;

    // handling the cors service
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: vec![Post, Get].into_iter().map(From::from).collect(),
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .mount("/", routes![hello])
        .mount("/api/auth", routes![login, register])
        .mount("/api/rss", routes![info, subscribe])
        .attach(DbConn::fairing())
        .attach(cors)
        .launch()
        .await
        .unwrap();
}
