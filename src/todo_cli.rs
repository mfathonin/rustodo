use console::style;
use dialoguer::{MultiSelect, Select};

use crate::todo::{Todo, TodoCollection};

/// Print the list of todo item available
/// `[x]` means done
pub fn list_todo(todo_col: &TodoCollection) {
    if todo_col.todos.len() == 0 {
        println!(
            "{}",
            style("Nothing here. Lets start the day!!").green().bold()
        );
        return;
    }

    println!("{}", style("Your todo list:").green().bold());
    for (_, todo) in todo_col.todos.iter().enumerate() {
        todo.print_todo();
    }
}

/// Display apps interface
pub fn show_menus(todo_col: &mut TodoCollection) -> u8 {
    let options = [
        "Toggle todo list",
        "Sort",
        "Remove done items",
        "Delete todo",
    ];
    list_todo(&todo_col);
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
            let id = todo_col.get_next_index();
            let todo = Todo::new(id, title);

            todo_col.add_todo(todo);

            retry
        }
    };

    return option;
}

pub fn mark_todo(todo_col: &mut TodoCollection) {
    if todo_col.todos.len() == 0 {
        return;
    }

    let items = todo_col
        .todos
        .iter()
        .map(|x| format!("[{}] {}", if x.done { "x" } else { "_" }, x.title))
        .collect::<Vec<String>>();

    let selected = Select::new()
        .with_prompt(style("Your todo list").green().bold().to_string())
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    todo_col.toggle_todo(selected);
}

pub fn delete_todo(todo_col: &mut TodoCollection) {
    if todo_col.todos.len() == 0 {
        return;
    }

    let items = todo_col
        .todos
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
        todo_col.remove_todo(idx);
    }
}
