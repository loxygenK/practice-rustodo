use crate::domain::{Todo, Tag};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RESTTodoScheme {
    id: String,
    name: String,
    memo: String,
    tags: Vec<RESTTagScheme>,
}
impl From<Todo> for RESTTodoScheme {
    fn from(todo: Todo) -> Self {
        RESTTodoScheme {
            id: todo.id,
            name: todo.name,
            memo: todo.memo,
            tags: todo.tags.into_iter().map(Into::into).collect::<Vec<RESTTagScheme>>()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RESTTagScheme {
    id: String,
    name: String,
    color: String,
}
impl From<Tag> for RESTTagScheme {
    fn from(tag: Tag) -> Self {
        RESTTagScheme {
            id: tag.id,
            name: tag.name,
            color: tag.color
        }
    }
}
