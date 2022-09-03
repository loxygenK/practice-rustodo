use crate::domain::Todo;

pub trait TodoRepository {
    type Error;

    fn get(&mut self, id: &str) -> Result<Vec<Todo>, Self::Error>;
}
