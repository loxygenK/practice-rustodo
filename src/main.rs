pub mod db;
pub mod repository;
mod domain;

use crate::{db::DbSession, repository::postgres::{todo::GetTodoQuery, DomainCompatibleQuery}};


fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL to be supplied");

    let mut connection = DbSession::establish(&database_url).expect("Fail");

    println!(
        "{:#?}",
        GetTodoQuery::new(&mut connection, "1740f5e5-91b5-467a-8e2d-a2c14cbae9e6").load()
    );

    // get_todo(&mut connection, "1740f5e5-91b5-467a-8e2d-a2c14cbae9e6");
}
