use anyhow::Context;
use crate::repository::{Repositories, traits::todo::TodoRepository};
use crate::domain::Todo;

pub fn get_todo(repos: &mut Repositories, id: &str) -> anyhow::Result<Todo> {
    repos.todo().get(id).context("There was an failure in DB")
}
