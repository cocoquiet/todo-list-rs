use rocket::{form::Form, get, post, response::Redirect};
use rocket_dyn_templates::Template;

use crate::models::{DeleteForm, NewTodo, Todo};

#[get("/")]
pub fn index() -> Template {}

#[get("/details/<id>")]
pub fn get_detail(id: i32) -> Template {}

#[get("/add")]
pub fn add() -> Template {}

#[post(
    "/add",
    format = "application/x-www-form-urlencoded",
    data = "<new_todo>"
)]
pub fn add_todo(new_todo: Form<NewTodo>) -> Redirect {}

#[post(
    "/delete",
    format = "application/x-www-form-urlencoded",
    data = "<form>"
)]
pub fn delete_form(form: Form<DeleteForm>) -> Redirect {}

#[get("/update/<id>")]
pub fn update(id: i32) -> Template {}

#[post(
    "/update",
    format = "application/x-www-form-urlencoded",
    data = "<update_todo>"
)]
pub fn update_todo(update_todo: Form<Todo>) -> Redirect {}

#[get("/error")]
pub fn error() -> Template {}
