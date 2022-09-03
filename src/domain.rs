pub struct Todo {
    id: String,
    name: String,
    memo: String,
    tags: Vec<Tag>,
}

impl Todo {
    pub fn new(id: String, name: String, memo: String, tags: Vec<Tag>) -> Self {
        Self { id, name, memo, tags, }
    }
}

pub struct Tag {
    id: String,
    name: String,
}

impl Tag {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
