use crate::domain::{Todo, Tag};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RESTTodoSchema {
    id: String,
    name: String,
    memo: String,
    tags: Vec<RESTTagSchema>,
}
impl From<Todo> for RESTTodoSchema {
    fn from(todo: Todo) -> Self {
        RESTTodoSchema {
            id: todo.id,
            name: todo.name,
            memo: todo.memo,
            tags: todo.tags.into_iter().map(Into::into).collect::<Vec<RESTTagSchema>>()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RESTTagSchema {
    id: String,
    name: String,
    color: String,
}
impl From<Tag> for RESTTagSchema {
    fn from(tag: Tag) -> Self {
        RESTTagSchema {
            id: tag.id,
            name: tag.name,
            color: tag.color
        }
    }
}
