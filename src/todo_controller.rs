use crate::todo::{
    model::{NewTodo, Todo},
    repository as repo,
};
use console::style;
use dialoguer::{MultiSelect, Select};
use diesel::PgConnection;

pub struct TodoController {
    conn: PgConnection,
}

impl TodoController {
    pub fn new(conn: PgConnection) -> Self {
        TodoController { conn }
    }

    pub fn add_todos(&mut self, todos_title: &[String]) {
        for title in todos_title {
            let new_todo = NewTodo::from(title.trim());
            repo::add_todo(&new_todo, &mut self.conn);
        }
    }

    pub fn print_with_id(&mut self) {
        let todos = repo::get_todos(&mut self.conn);

        if todos.len() == 0 {
            println!("{}", style("Nothing here\n").green().bold());
            return;
        }

        for todo in todos {
            let title: String = if todo.done {
                style(todo.title).strikethrough().to_string()
            } else {
                todo.title
            };

            println!("  [{}]\t{}", todo.id, title);
        }
        println!("");
    }

    /// Print the list of todo item available
    /// `[x]` means done
    pub fn list_todo(&mut self) {
        let todos = repo::get_todos(&mut self.conn);
        if todos.len() == 0 {
            println!(
                "{}",
                style("Nothing here. Lets start the day!!").green().bold()
            );
            return;
        }

        println!("{}", style("Your todo list:").green().bold());
        for (_, todo) in todos.iter().enumerate() {
            todo.print_todo();
        }
    }

    /// Display apps interface
    pub fn show_menus(&mut self) -> u8 {
        let options = ["Toggle todo list", "Remove done items", "Delete todo"];
        self.list_todo();
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
                let retry: u8 = options.len() as u8 + 1;

                if option.trim() == "" {
                    return retry;
                }

                let title = option.trim();
                let new_todo = NewTodo::from(title);
                repo::add_todo(&new_todo, &mut self.conn);

                retry
            }
        };

        return option;
    }

    pub fn toggle_todo(&mut self, ids: Vec<i32>) {
        for id in ids {
            repo::toggle_todo(&id, &mut self.conn);
        }
    }

    pub fn mark_todo(&mut self) {
        let todos = repo::get_todos(&mut self.conn);
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

        self.toggle_todo(vec![todos[selected].id]);
    }

    pub fn remove_done(&mut self) {
        repo::clean_todo(&mut self.conn);
    }

    pub fn delete_todo_by_id(&mut self, ids: Vec<i32>) {
        for id in ids {
            repo::remove_todo(&id, &mut self.conn);
        }
    }

    pub fn delete_todo(&mut self) {
        let todos: Vec<Todo> = repo::get_todos(&mut self.conn);
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
            let id = todos[idx].id;
            repo::remove_todo(&id, &mut self.conn);
        }
    }
}
