use anyhow::Context;
use crate::repository::{Repositories, traits::todo::TodoRepository};
use crate::domain::Todo;

pub fn get_todo(repos: &mut Repositories, id: &str) -> anyhow::Result<Todo> {
    repos.todo().get(id).context("There was an failure in DB")
}

pub fn fixed_todo(repos: &mut Repositories) -> anyhow::Result<Todo> {
    repos.todo().get("1740f5e5-91b5-467a-8e2d-a2c14cbae9e6").context("There was an failure in DB")
}
