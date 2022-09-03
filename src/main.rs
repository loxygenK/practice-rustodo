use crate::repository::{postgres::{todo::PostgresTodoRepository, init::DbSession}, traits::todo::TodoRepository};

pub mod repository;
mod domain;

fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL to be supplied");

    let session = DbSession::establish(&database_url).expect("Fail");
    let mut repo = PostgresTodoRepository::new(session);

    println!("{:#?}", repo.get("1740f5e5-91b5-467a-8e2d-a2c14cbae9e6"));
}
