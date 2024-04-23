mod schema;
mod todo;
mod todo_controller;
mod tools;

use console::Term;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::env;

use todo_controller::TodoController;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable not set");
    let connections = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to the DB (postgres).\nPlease check your `DATABASE_URL` environment variable"));

    let args: Vec<String> = env::args().collect();

    let mut todo_controller = TodoController::new(connections);

    if args.len() > 1 {
        tools::tools(&args, &mut todo_controller);
        return;
    }

    let term = Term::stdout();
    clear_screen(&term);
    let mut opt = todo_controller.show_menus();

    loop {
        if opt == 0 {
            break;
        };

        clear_screen(&term);
        match opt {
            1 => todo_controller.mark_todo(),
            2 => todo_controller.remove_done(),
            3 => todo_controller.delete_todo(),
            0 => break,
            _ => println!("\nWrong options"),
        };

        clear_screen(&term);
        opt = todo_controller.show_menus();
    }
}

fn clear_screen(term: &Term) {
    let _ = term.clear_screen();
}
