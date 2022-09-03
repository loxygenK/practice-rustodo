use thiserror::Error;

use diesel::{Connection, PgConnection};

#[derive(Error, Debug)]
#[error(transparent)]
pub struct ConnectionError(diesel::ConnectionError);

pub struct DbSession {
    pub(super) connection: PgConnection,
}
impl DbSession {
    /// Establishes the connection between the database and the application.
    pub fn establish(database_url: &str) -> Result<DbSession, ConnectionError> {
        let connection = PgConnection::establish(database_url).map_err(ConnectionError)?;

        Ok(Self { connection })
    }
}
