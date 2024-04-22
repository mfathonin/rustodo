mod todo;
mod todo_cli;

use console::Term;
use todo::TodoCollection;

fn main() {
    let term = Term::stdout();

    let mut todo_collection = TodoCollection::new();

    clear_screen(&term);

    let mut opt = todo_cli::show_menus(&mut todo_collection);

    loop {
        if opt == 0 {
            break;
        };

        clear_screen(&term);
        match opt {
            1 => todo_cli::mark_todo(&mut todo_collection),
            2 => todo_collection.sort_todo(),
            3 => todo_collection.remove_done(),
            4 => todo_cli::delete_todo(&mut todo_collection),
            0 => break,
            _ => println!("\nWrong options"),
        };

        clear_screen(&term);
        opt = todo_cli::show_menus(&mut todo_collection);
    }
}

fn clear_screen(term: &Term) {
    let _ = term.clear_screen();
}
