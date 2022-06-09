mod router;
mod types;
mod utility;
mod error;
mod database;
mod constant;
mod core;
mod rss;

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;

use rocket::{
    http::Method::{Get, Post}, 
    routes, 
    tokio::{self, time::{self, Duration}},
};
use rocket_cors::AllowedOrigins;
use rocket_sync_db_pools::database;
use router::{
    auth::{login, register},
    rss::{ info, subscribe, unsubscribe},
    crates::{subscribed, refresh_message, read_message, unread_message}
};
use env_logger::Env;

#[database("sqlite_rss")]
pub struct DbConn(diesel::SqliteConnection);

#[rocket::main]
async fn main() {
    // 初始化日志服务
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    
    tokio::spawn(core::check_update());
    time::sleep(Duration::new(2, 0)).await;
    
    tokio::spawn(core::notify_launch());
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
        .mount("/api/auth", routes![login, register])
        .mount("/api/rss", routes![info, subscribe, unsubscribe])
        .mount("/api/crate", routes![subscribed, refresh_message, read_message, unread_message])
        .attach(DbConn::fairing())
        .attach(cors)
        .launch()
        .await
        .unwrap();
}

