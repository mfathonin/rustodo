use std::io;

struct Todo {
    id: usize,
    title: String,
    done: bool,
}

static mut LAST_IDX: usize = 0;

fn main() {
    let mut todos = Vec::new();

    let mut is_loop = true;

    while is_loop {
        let option = apps_interface();

        match option {
            1 => list_todo(&todos),
            2 => create_todo(&mut todos),
            3 => mark_todo(&mut todos),
            0 => is_loop = false,
            _ => println!("\nWrong options"),
        }
    }
}

/// Display apps interface
fn apps_interface() -> u8 {
    println!("\nOptions:");
    println!("1. Show todo list\n2. Add todo list\n3. Toggle todo list");
    println!("\nPlease select your options: (0 for exit)");
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Error read line");
    let option: u8 = match option.trim().parse() {
        Ok(op) => op,
        Err(_) => 0,
    };

    return option;
}

/// Creating Todo from user via interactive terminal
fn create_todo(todos: &mut Vec<Todo>) {
    println!("Todo title:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Error read line");
    let title = title.trim().to_string();

    unsafe {
        let td = Todo {
            id: LAST_IDX,
            title,
            done: false,
        };
        LAST_IDX += 1;
        add_todo(td, todos)
    }
}

/// Add todo to the Todo-list
///
/// _In this version we still need to pass todos vector as 2nd parameter_
fn add_todo(todo: Todo, todos: &mut Vec<Todo>) {
    println!("Todo added: {}", todo.title);
    todos.push(todo);
}

/// Print the list of todo item available
/// `[x]` means done
fn list_todo(todos: &Vec<Todo>) {
    println!("Your todolist:");
    for todo in todos {
        let is_done = if todo.done { "x" } else { "_" };
        println!("[{}] {}. {}", is_done, todo.id + 1, todo.title);
    }
}

fn mark_todo(todos: &mut Vec<Todo>) {
    loop {
        println!("Select todo by index:");
        let mut idx = String::new();
        io::stdin()
            .read_line(&mut idx)
            .expect("Error reading index");
        let idx: usize = idx.trim().parse().expect("Error parsing index");

        if idx >= todos.len() {
            println!("Out off scope");
        } else {
            toggle_todo(idx, todos);
            list_todo(todos);

            break;
        }
    }
}

/// Toggle todo status by id
///
/// _In current version we need to pass todos vector as 2nd parameter_
fn toggle_todo(id: usize, todos: &mut Vec<Todo>) {
    todos[id].done = !todos[id].done;
}
