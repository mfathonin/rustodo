use core::panic;

use super::model::{NewTodo, Todo};
use crate::schema::todos::{dsl::*, table};
use diesel::prelude::*;

pub fn get_todos(conn: &mut PgConnection) -> Vec<Todo> {
    let res = todos
        .select(Todo::as_select())
        .order(done.asc())
        .load(conn)
        .expect("Error loading todo from DB");

    return res;
}

pub fn add_todo(todo: &NewTodo, conn: &mut PgConnection) {
    diesel::insert_into(table)
        .values(todo)
        .execute(conn)
        .expect("Error adding new todo");
}

pub fn get_todo_by_id(todo_id: &i32, conn: &mut PgConnection) -> Option<Todo> {
    todos
        .find(todo_id)
        .select(Todo::as_select())
        .first(conn)
        .optional()
        .expect(format!("Can't get todo with id: {}", todo_id).trim())
}

pub fn toggle_todo(todo_id: &i32, conn: &mut PgConnection) {
    let cur_todo = match get_todo_by_id(todo_id, conn) {
        Some(todo) => todo,
        None => panic!("No todo with id: {} found", todo_id),
    };

    let updated_todo = diesel::update(todos.find(todo_id))
        .set(done.eq(!cur_todo.done))
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error update todo");

    println!("{} is {}", updated_todo.title, updated_todo.done);
}

pub fn remove_todo(todo_id: &i32, conn: &mut PgConnection) {
    diesel::delete(todos.find(todo_id))
        .execute(conn)
        .expect(format!("Error while deleting todo with id: {}", todo_id).trim());
}

pub fn clean_todo(conn: &mut PgConnection) {
    diesel::delete(todos.filter(done.eq(true)))
        .execute(conn)
        .expect("Error deleting todos that has been done");
}
