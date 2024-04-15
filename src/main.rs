mod todo;
mod todo_cli;

use console::Term;
use todo::{remove_done, sort_todo};

fn main() {
    let term = Term::stdout();

    let mut todos = Vec::new();

    let _ = term.clear_screen();

    let mut option = todo_cli::show_menus(&mut todos);

    loop {
        if option == 0 {
            break;
        }

        let _ = term.clear_screen();
        match option {
            1 => todo_cli::mark_todo(&mut todos),
            2 => {
                sort_todo(&mut todos);
            }
            3 => todos = remove_done(todos),
            4 => todo_cli::delete_todo(&mut todos),
            0 => break,
            _ => println!("\nWrong options"),
        }

        let _ = term.clear_screen();
        option = todo_cli::show_menus(&mut todos);
    }
}
