use diesel::Queryable;

use crate::types::database::{ task, crates };

#[derive(Insertable, AsChangeset)]
#[table_name = "crates"]
pub struct CratesQuery {
    pub crates_id: String,
    pub crates_type: String,
    pub info: String
}


#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "task"]
pub struct TaskQuery {
    pub id: Option<i32>,
    pub crates_id: String,
    pub task_type: String,
    pub username: String,
    pub params: String

}