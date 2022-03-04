use rocket::serde::{ Serialize, Deserialize};
use crate::types::database::{ User, login_state};

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
#[table_name="User"]
pub struct AccountInfo {
    pub username: String,
    pub passwd: String 
}


#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name="User"]
pub struct UserQuery {
    pub id: Option<i32>,
    pub username: String,
    pub passwd: String
}

#[derive(Insertable, AsChangeset)]
#[table_name="login_state"]
pub struct LoginStateInfo {
    pub username: String,
    pub session_data: String
}