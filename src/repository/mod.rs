use self::{traits::todo::TodoRepository, postgres::todo::PostgresRepositoriesSet};

pub mod postgres;
pub mod traits;

pub enum Repositories {
    Postgres(PostgresRepositoriesSet)
}
impl Repositories {
    pub fn todo(&mut self) -> &mut impl TodoRepository {
        match self {
            Repositories::Postgres(p) => &mut p.todo
        }
    }
}

pub struct RepositoriesSet<Todo: TodoRepository> {
    pub todo: Todo
}
