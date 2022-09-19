use crate::domain::Todo;

pub trait TodoRepository {
    type Error: std::error::Error + Send + Sync;

    fn get(&mut self, id: &str) -> Result<Todo, Self::Error>;
}
