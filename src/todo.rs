#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub done: bool,
}

/// Add todo to the Todo-list
///
/// _In this version we still need to pass todos vector as 2nd parameter_
pub fn add_todo(todo: Todo, todos: &mut Vec<Todo>) {
    todos.push(todo);
}

/// Print the list of todo item available
/// `[x]` means done
pub fn list_todo(todos: &Vec<Todo>) {
    if todos.len() == 0 {
        println!("Nothing here. Lets start the day!!");
        return;
    }

    println!("Your todolist:");
    for todo in todos {
        let is_done = if todo.done { "x" } else { "_" };
        println!("[{}] {}", is_done, todo.title);
    }
}

/// Toggle todo status by id
///
/// _In current version we need to pass todos vector as 2nd parameter_
pub fn toggle_todo(id: usize, todos: &mut Vec<Todo>) {
    todos[id].done = !todos[id].done;
}

/// Get last index available for adding new todo
pub fn get_next_index(todos: &Vec<Todo>) -> usize {
    if todos.len() == 0 {
        return 0;
    }

    let mut max: usize = 0;
    for t in todos {
        if t.id > max {
            max = t.id;
        }
    }
    return max + 1;
}

/// Sorting todo list
pub fn sort_todo(todos: &mut Vec<Todo>) {
    todos.sort_by(|a, b| a.done.cmp(&b.done));
}

/// Remove done items
pub fn remove_done(todos: Vec<Todo>) -> Vec<Todo> {
    let todos: Vec<Todo> = todos
        .into_iter()
        .filter(|t| t.done == false)
        .collect::<Vec<Todo>>();
    return todos;
}

pub fn remove_todo(idx: usize, todos: &mut Vec<Todo>) {
    todos.remove(idx);
}
