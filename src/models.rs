use diesel::prelude::*;
use rocket::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, FromForm)]
#[diesel(table_name=crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: String
}

#[derive(Insertable, Serialize,, FromForm)]
#[diesel(table_name=crate::schema::todos)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
    pub completed: bool
}

#[derive(FromForm)]
pub struct DeleteForm {
    pub id: i32
}