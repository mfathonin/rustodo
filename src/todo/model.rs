use diesel::prelude::*;
use serde::Serialize;

use crate::schema::todos;

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub title: String,
    pub done: bool,
}

impl NewTodo {
    pub fn from(title: &str) -> Self {
        Self {
            title: String::from(title),
            done: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Selectable)] // <- Attributes: Docs at easy-rust #28
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
}
