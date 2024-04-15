mod todo;
mod todo_cli;

use console::Term;
use todo::{add_todo, remove_done, sort_todo};

fn main() {
    let term = Term::stdout();

    let mut todos = Vec::new();

    let mut is_loop = true;

    let _ = term.clear_screen();

    let mut option = todo_cli::show_menus(&todos);

    while is_loop {
        if option == 0 {
            break;
        }

        let _ = term.clear_screen();
        match option {
            1 => {
                let todo = todo_cli::add_todo(&todos);
                add_todo(todo, &mut todos);
            }
            2 => todo_cli::mark_todo(&mut todos),
            3 => {
                sort_todo(&mut todos);
            }
            4 => todos = remove_done(todos),
            5 => todo_cli::delete_todo(&mut todos),
            0 => is_loop = false,
            _ => println!("\nWrong options"),
        }

        let _ = term.clear_screen();
        option = todo_cli::show_menus(&todos);
    }
}
