use thiserror::Error;

use diesel::{PgConnection, Connection};

#[derive(Error, Debug)]
#[error("DB connection failed!: {source}")]
pub struct ConnectionError {
    source: diesel::ConnectionError
}

pub struct DbSession {
    connection: PgConnection
}
impl DbSession {
    /// Establishes the connection between the database and the application.
    pub fn establish(database_url: &str) -> Result<DbSession, ConnectionError> {
        let connection = PgConnection::establish(database_url).map_err(|e| ConnectionError { source: e })?;

        Ok(Self { connection })
    }
}
