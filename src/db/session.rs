use std::{fmt::Display, error::Error};

use diesel::{PgConnection, Connection};

#[derive(Debug)]
pub struct ConnectionError {
    source: diesel::ConnectionError
}
impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The problem has occured during establishing connection between database: {:#?}", self.source);

        Ok(())
    }
}
impl Error for ConnectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
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
