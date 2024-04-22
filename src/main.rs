mod schema;
mod todo;
mod todo_operator;

use std::env;
use todo_operator::TodoOperator;

use console::Term;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable not set");
    let connections = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url));

    let term = Term::stdout();

    // clear_screen(&term);

    let mut todo_op = TodoOperator::new(connections);

    let mut opt = todo_op.show_menus();

    loop {
        if opt == 0 {
            break;
        };

        clear_screen(&term);
        match opt {
            1 => todo_op.mark_todo(),
            2 => todo_op.remove_done(),
            3 => todo_op.delete_todo(),
            0 => break,
            _ => println!("\nWrong options"),
        };

        clear_screen(&term);
        opt = todo_op.show_menus();
    }
}

fn clear_screen(term: &Term) {
    let _ = term.clear_screen();
}
