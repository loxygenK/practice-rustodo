mod domain;
pub mod db;

fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL to be supplied");

    let connection = db::session::DbSession::establish(&database_url);

    println!("{:#?}", connection.err());
}
