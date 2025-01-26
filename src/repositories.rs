use crate::{models::{NewTodo, Todo}, schema::todos::{self}};
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::form::Form;
use std::env;

pub struct TodoRepositories;

impl TodoRepositories {
    pub fn extablish_connection() {}

    pub fn show_all() {}

    pub fn show() {}

    pub fn create() {}

    pub fn update() {}

    pub fn delete() {}
}