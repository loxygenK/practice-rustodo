pub mod get;

use crate::{DbSession, repository::{traits::todo::TodoRepository, Repositories, RepositoriesSet}, domain::Todo};
use self::get::GetTodoQuery;
use super::{QueryError, DomainCompatibleQuery};

pub type PostgresRepositoriesSet = RepositoriesSet<PostgresTodoRepository>;
pub fn init_postgres_repo(db_session: DbSession) -> Repositories {
    Repositories::Postgres(RepositoriesSet {
        todo: PostgresTodoRepository(db_session)
    })
}

pub struct PostgresTodoRepository(DbSession);

impl TodoRepository for PostgresTodoRepository {
    type Error = QueryError;

    fn get(&mut self, id: &str) -> Result<Todo, Self::Error> {
        GetTodoQuery::new(&mut self.0, id).load()
    }
}

impl PostgresTodoRepository {
    pub fn new(session: DbSession) -> Self {
        Self(session)
    }
}
