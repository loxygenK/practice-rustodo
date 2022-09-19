pub mod context;
pub mod todo;
pub mod scheme;
pub mod error;

use warp::Filter;

use crate::{repository::Repositories, helpers::new_shared};

use self::todo::todo_route;

pub async fn generate_rest_endpoint(session: Repositories) {
    let shared_session = new_shared(session);

    let routes = warp::any()
        .and(todo_route(shared_session.clone()));
        //.and(other)

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
