

#[derive(Clone, Debug, PartialEq)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Clone, Debug, Default)]
pub struct TodoState {
    pub items: Vec<Todo>,
    pub last_id: u32,
}

impl TodoState {
    pub fn add(&mut self, title: impl Into<String>) -> u32 {
        let id = self.last_id + 1;
        self.last_id = id;
        self.items.push(Todo {
            id,
            title: title.into(),
            done: false,
        });
        id
    }

    pub fn toggle(&mut self, id: u32) {
        if let Some(todo) = self.items.iter_mut().find(|todo| todo.id == id) {
            todo.done = !todo.done;
        }
    }

    pub fn delete(&mut self, id: u32) {
        self.items.retain(|todo| todo.id != id);
    }
}