use crate::domain::Todo;
use crate::endpoints::rest::error::reject_anyhow;
use crate::endpoints::rest::scheme::RESTTodoScheme;
use crate::helpers::Shared;
use crate::repository::Repositories;
use crate::service;

pub async fn get_todo(session: Shared<Repositories>, id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let mut session = session.lock_owned().await;

    let todo: Todo = service::todo::get_todo(&mut session, &id).map_err(reject_anyhow)?;
    let rest_todo: RESTTodoScheme = todo.into();

    Ok(warp::reply::json(&rest_todo))
}
