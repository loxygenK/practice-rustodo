use repository::postgres::todo::init_postgres_repo;

use crate::repository::postgres::init::DbSession;
use crate::endpoints::rest::generate_rest_endpoint;

pub mod repository;
pub mod domain;
pub mod endpoints;
pub mod service;
pub mod helpers;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL to be supplied");

    let session = DbSession::establish(&database_url).expect("Fail");
    let repos = init_postgres_repo(session);

    // println!("{:#?}", repo.get("1740f5e5-91b5-467a-8e2d-a2c14cbae9e6"));

    generate_rest_endpoint(repos).await;
}
