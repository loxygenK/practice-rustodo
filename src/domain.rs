#[derive(Debug)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub memo: String,
    pub tags: Vec<Tag>,
}

impl Todo {
    pub fn new(id: &str, name: &str, memo: &str, tags: Vec<Tag>) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            memo: memo.to_string(),
            tags,
        }
    }
}

#[derive(Debug)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
}

impl Tag {
    pub fn new(id: &str, name: &str, color: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            color: color.to_string(),
        }
    }
}
