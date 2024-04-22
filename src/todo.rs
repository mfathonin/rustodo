use console::style;

#[derive(Debug, Clone)] // <- Attributes: Docs at easy-rust #28
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub done: bool,
}

/// This is impl for Todo struct
/// This will allow user to use like
/// ```rust
/// let mut todo = Todo::new(id, title);
/// todo.toggle();
///
/// println!("{:?}", todo); // <- {:?} this allowed by adding attribut `derive(Debug)`
/// ```
impl Todo {
    pub fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            done: false,
        }
    }

    pub fn toggle(&mut self) {
        self.done = !self.done;
    }

    pub fn print_todo(&self) {
        if self.done {
            println!("  [x] {}", style(&self.title).strikethrough());
        } else {
            println!("  [_] {}", self.title);
        }
    }
}

pub struct TodoCollection {
    pub todos: Vec<Todo>,
}

impl TodoCollection {
    pub fn new() -> Self {
        Self { todos: vec![] }
    }

    /// Add todo to the Todo-list
    ///
    /// _In this version we still need to pass todos vector as 2nd parameter_
    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    /// Toggle todo status by id
    ///
    /// _In current version we need to pass todos vector as 2nd parameter_
    pub fn toggle_todo(&mut self, id: usize) {
        self.todos[id].toggle();
    }

    /// Get last index available for adding new todo
    pub fn get_next_index(&self) -> usize {
        if self.todos.len() == 0 {
            return 0;
        }

        let mut max: usize = 0;
        for t in &self.todos {
            if t.id > max {
                max = t.id;
            }
        }
        return max + 1;
    }

    /// Sorting todo list
    pub fn sort_todo(&mut self) {
        self.todos.sort_by(|a, b| a.done.cmp(&b.done));
    }

    /// Remove done items
    pub fn remove_done(&mut self) {
        let todos: Vec<Todo> = self
            .todos
            .clone()
            .into_iter()
            .filter(|t| t.done == false)
            .collect::<Vec<Todo>>();
        self.todos = todos;
    }

    /// Remove todo by index
    pub fn remove_todo(&mut self, idx: usize) {
        self.todos.remove(idx);
    }
}
