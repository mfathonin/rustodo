use crate::todo::{get_next_index, list_todo, remove_todo, toggle_todo, Todo};

/// Display apps interface
pub fn show_menus(todos: &Vec<Todo>) -> u8 {
    list_todo(&todos);
    println!("\nOptions:");
    println!("1. Add todo list");
    println!("2. Toggle todo list");
    println!("3. Sort");
    println!("4. Remove done items");
    println!("5. Delete todo");

    println!("\nPlease select your options: (0 for exit)");
    let mut option = String::new();
    std::io::stdin()
        .read_line(&mut option)
        .expect("Error read line");
    let option: u8 = match option.trim().parse() {
        Ok(op) => op,
        Err(_) => 0,
    };

    return option;
}

pub fn add_todo(todos: &Vec<Todo>) -> Todo {
    println!("New todo item:");
    let mut title = String::new();
    std::io::stdin()
        .read_line(&mut title)
        .expect("Error read todo title");
    let title = title.trim().to_string();

    let id = get_next_index(todos);

    return Todo {
        id,
        title,
        done: false,
    };
}

pub fn mark_todo(todos: &mut Vec<Todo>) {
    if todos.len() == 0 {
        return;
    }

    loop {
        list_todo(todos);
        println!("\nSelect todo by index (start from 0):");
        let mut idx = String::new();
        std::io::stdin()
            .read_line(&mut idx)
            .expect("Error reading index");
        let idx: usize = idx.trim().parse().expect("Error parsing index");

        if idx >= todos.len() {
            println!("Out off scope");
        } else {
            toggle_todo(idx, todos);

            break;
        }
    }
}

pub fn delete_todo(todos: &mut Vec<Todo>) {
    if todos.len() == 0 {
        return;
    }

    loop {
        list_todo(&todos);
        println!("\nSelect todo by index (start from 0):");
        let mut idx = String::new();
        std::io::stdin()
            .read_line(&mut idx)
            .expect("Error reading index");
        let idx: usize = idx.trim().parse().expect("Error parsing index");

        if idx >= todos.len() {
            println!("Out of scope");
        } else {
            remove_todo(idx, todos);
            break;
        }
    }
}
