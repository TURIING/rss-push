use crate::types::database::{login_state, user};
use rocket::serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "user"]
pub struct AccountInfo {
    pub username: String,
    pub passwd: String,
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "user"]
pub struct UserQuery {
    pub id: Option<i32>,
    pub username: String,
    pub passwd: String,
}


#[derive(Insertable, AsChangeset)]
#[table_name = "login_state"]
pub struct LoginStateInfo {
    pub username: String,
    pub session_data: String,
}
