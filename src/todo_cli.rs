use console::style;
use dialoguer::{MultiSelect, Select};

use crate::todo::{add_todo, get_next_index, remove_todo, toggle_todo, Todo};

/// Print the list of todo item available
/// `[x]` means done
pub fn list_todo(todos: &Vec<Todo>) {
    if todos.len() == 0 {
        println!(
            "{}",
            style("Nothing here. Lets start the day!!").green().bold()
        );
        return;
    }

    println!("{}", style("Your todo list:").green().bold());
    for todo in todos {
        if todo.done {
            println!("  [x] {}", style(&todo.title).strikethrough());
        } else {
            println!("  [_] {}", todo.title);
        }
    }
}

/// Display apps interface
pub fn show_menus(todos: &mut Vec<Todo>) -> u8 {
    let options = [
        "Toggle todo list",
        "Sort",
        "Remove done items",
        "Delete todo",
    ];
    list_todo(&todos);
    println!("\n{}", style("Options:").bold().cyan());
    for (id, op) in options.iter().enumerate() {
        println!("  {}. {}", style(id + 1).cyan(), op);
    }
    println!("\nAdd new task or type option number (`0` for exit)");
    let mut option = String::new();
    std::io::stdin()
        .read_line(&mut option)
        .expect("Error read line");
    let option: u8 = match option.trim().parse() {
        Ok(op) => op,
        Err(_) => {
            let retry: u8 = option.len() as u8 + 1;

            if option.trim() == "" {
                return retry;
            }

            let title = option.trim().to_string();
            let id = get_next_index(todos);
            let todo = Todo {
                id,
                title,
                done: false,
            };

            add_todo(todo, todos);

            retry
        }
    };

    return option;
}

pub fn mark_todo(todos: &mut Vec<Todo>) {
    if todos.len() == 0 {
        return;
    }

    let items = todos
        .iter()
        .map(|x| format!("[{}] {}", if x.done { "x" } else { "_" }, x.title))
        .collect::<Vec<String>>();

    let selected = Select::new()
        .with_prompt(style("Your todo list").green().bold().to_string())
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    toggle_todo(selected, todos);
}

pub fn delete_todo(todos: &mut Vec<Todo>) {
    if todos.len() == 0 {
        return;
    }

    let items = todos
        .iter()
        .map(|x| format!("{}", x.title))
        .collect::<Vec<String>>();

    let mut idxs = MultiSelect::new()
        .with_prompt(style("Your todo list").green().bold().to_string())
        .items(&items)
        .interact()
        .unwrap();

    idxs.reverse();

    for idx in idxs {
        remove_todo(idx, todos);
    }
}
