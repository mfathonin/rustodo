use console::style;

use self::model::Todo;

pub mod model;
pub mod repository;

/// This is impl for Todo struct
/// This will allow user to use like
/// ```rust
/// let mut todo = Todo::new(id, title);
/// todo.toggle();
///
/// println!("{:?}", todo); // <- {:?} this allowed by adding attribut `derive(Debug)`
/// ```
impl Todo {
    pub fn print_todo(&self) {
        if self.done {
            println!("  [x] {}", style(&self.title).strikethrough());
        } else {
            println!("  [_] {}", self.title);
        }
    }
}
