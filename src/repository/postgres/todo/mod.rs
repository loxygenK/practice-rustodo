pub mod get;

use crate::{DbSession, repository::traits::todo::TodoRepository, domain::Todo};

use self::get::GetTodoQuery;

use super::{QueryError, DomainCompatibleQuery};

pub struct PostgresTodoRepository(DbSession);

impl TodoRepository for PostgresTodoRepository {
    type Error = QueryError;

    fn get(&mut self, id: &str) -> Result<Vec<Todo>, Self::Error> {
        GetTodoQuery::new(&mut self.0, id).load()
    }
}

impl PostgresTodoRepository {
    pub fn new(session: DbSession) -> Self {
        Self(session)
    }
}
