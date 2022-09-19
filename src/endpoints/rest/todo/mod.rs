pub mod get;

use crate::helpers::Shared;
use crate::repository::Repositories;

use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use self::get::get_todo;

use super::context::with_db;

pub fn todo_route(shared_session: Shared<Repositories>) -> BoxedFilter<(impl Reply,)> {
    let todo_get = warp::get()
        .and(with_db(shared_session))
        .and(warp::path::param::<String>())
        .and_then(get_todo);

    warp::path("todo")
        .and(todo_get)
        .boxed()
}
