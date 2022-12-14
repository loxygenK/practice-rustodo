use diesel::result::Error;

mod schema;
pub mod todo;
pub mod init;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct QueryError(Error);

type QueryResult<T> = Result<T, QueryError>;

pub trait ResponseScheme {
    type Columns;
    fn columns() -> Self::Columns;
}

pub trait Query {
    type ResponseScheme: ResponseScheme;

    fn execute(&mut self) -> QueryResult<Vec<Self::ResponseScheme>>;
}

pub trait DomainCompatibleQuery: Query {
    type Domain;

    fn to_domain(response: &[Self::ResponseScheme]) -> Self::Domain;

    fn load(&mut self) -> QueryResult<Self::Domain> {
        let response = self.execute()?;

        Ok(Self::to_domain(&response))
    }
}
