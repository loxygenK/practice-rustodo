pub mod get;

use crate::domain::Todo;
use crate::helpers::Shared;
use crate::repository::Repositories;
use crate::service;

use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use super::context::with_db;
use super::error::reject_anyhow;
use super::schema::RESTTodoSchema;

pub fn todo_route(shared_session: &Shared<Repositories>) -> BoxedFilter<(impl Reply,)> {
    let todo_get = warp::get()
        .and(with_db(shared_session.clone()))
        .and(warp::path::param::<String>())
        .and_then(get_todo);

    let todo_fixed_get = warp::get()
        .and(with_db(shared_session.clone()))
        .and_then(get_fixed_todo);

    warp::path("todo")
        .and(todo_get)
        .or(todo_fixed_get)
        .boxed()
}

async fn get_todo(session: Shared<Repositories>, id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let mut session = session.lock_owned().await;

    let todo: Todo = service::todo::get_todo(&mut session, &id).map_err(reject_anyhow)?;
    let rest_todo: RESTTodoSchema = todo.into();

    Ok(warp::reply::json(&rest_todo))
}

async fn get_fixed_todo(session: Shared<Repositories>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut session = session.lock_owned().await;

    let todo: Todo = service::todo::fixed_todo(&mut session).map_err(reject_anyhow)?;
    Ok(warp::reply::json::<RESTTodoSchema>(&todo.into()))
}
