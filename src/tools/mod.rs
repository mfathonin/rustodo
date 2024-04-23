use console::style;

use crate::todo_operator::TodoOperator;

pub fn tools(args: &Vec<String>, todo_op: &mut TodoOperator) {
    let cmd = &args[1];

    match cmd.trim() {
        "add" => match check_min_args(&args, "Please provide minimum 1 todo item.") {
            Err(msg) => {
                println!("{msg}\n");
                show_help();
            }
            Ok(_) => {
                let list = &args[2..];
                todo_op.add_todos(list);
                todo_op.print_with_id();
            }
        },
        "list" => {
            todo_op.print_with_id();
        }
        "toggle" => match check_min_args(&args, "Need minimum 1 todo id.") {
            Ok(_) => {
                let list = &args[2..];
                let list: Vec<i32> = list
                    .iter()
                    .map(|id| match id.parse::<i32>() {
                        Ok(x) => x,
                        Err(_) => -1,
                    })
                    .filter(|x| *x >= 0)
                    .collect();
                todo_op.toggle_todo(list);
                todo_op.print_with_id();
            }
            Err(msg) => {
                println!("{msg}\n");
                show_help();
            }
        },
        "delete" => match check_min_args(&args, "Need minimum 1 todo id.") {
            Ok(_) => {
                let list = &args[2..];
                let list: Vec<i32> = list
                    .iter()
                    .map(|id| match id.parse::<i32>() {
                        Ok(i) => i,
                        Err(_) => -1,
                    })
                    .filter(|i| *i >= 0)
                    .collect();
                todo_op.delete_todo_by_id(list);
                todo_op.print_with_id();
            }
            Err(msg) => {
                println!("{msg}\n");
                show_help();
            }
        },
        _ => show_help(),
    }
}

fn check_min_args(args: &Vec<String>, msg: &str) -> Result<bool, String> {
    if args.len() < 3 {
        let title = style("Invalid command.").red().bold().to_string();
        let msg = style(msg).red().to_string();
        let err_msg = format!("{title}\n{msg}");
        return Err(err_msg);
    }

    return Ok(true);
}

fn show_help() {
    println!("{}", style("Available commands are:").green());
    println!(" - add [title|s]\tAdd new todo item. min 1 title must be provided. wrap with \"\" (double quote) for long title");
    println!(" - toggle [id|s]\tToggle todo status from provided id(s). min 1 id must be provided");
    println!(" - delete [id|s]\tDelete todo with provided id(s). min 1 id must be provided");
    println!(" - list\t\t\tshow all todo list");
    println!(" - help\t\t\tshow this help docs");
}
