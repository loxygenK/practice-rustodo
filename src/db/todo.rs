use diesel::prelude::*;
use itertools::Itertools;

use crate::domain::{Todo, Tag};

use super::{ResponseScheme, Query, QueryError, DomainCompatibleQuery};
use super::session::DbSession;
use super::schema::todos::{self, dsl::{*, id as todo_id}};
use super::schema::tags::{self, dsl::*};
use super::schema::rel_todos_tags::dsl::*;
use super::QueryResult;

// TODO: Can I make this more structure-ish? (Like nesting structure, no so much neccesary though)
#[derive(Debug, Queryable)]
pub struct GetTodoQueryResponse {
    todo_id: String,
    todo_name: String,
    todo_memo: Option<String>,
    tag_id: String,
    tag_name: String,
    tag_color: String
}

impl ResponseScheme for GetTodoQueryResponse {
    type Columns = (todos::id, todos::name, todos::memo, tags::id, tags::name, tags::color);

    fn columns() -> Self::Columns {
        (todos::id, todos::name, todos::memo, tags::id, tags::name, tags::color)
    }
}

pub struct GetTodoQuery<'a>(&'a mut DbSession, String);
impl<'a> GetTodoQuery<'a> {
    pub fn new(session: &'a mut DbSession, query_id: &str) -> Self {
        Self(session, query_id.to_string())
    }
}

impl<'a> Query for GetTodoQuery<'a> {
    type ResponseScheme = GetTodoQueryResponse;

    fn execute(&mut self) -> QueryResult<Vec<Self::ResponseScheme>> {
        let conn = &mut self.0;
        let query_id = &self.1;

        todos
            .inner_join(rel_todos_tags.inner_join(tags))
            .filter(todo_id.eq(query_id))
            .select(GetTodoQueryResponse::columns())
            .load::<GetTodoQueryResponse>(&mut conn.connection)
            .map_err(QueryError)
    }
}

impl<'a> DomainCompatibleQuery for GetTodoQuery<'a> {
    type Domain = Todo;

    fn to_domain(responses: &[Self::ResponseScheme]) -> Vec<Self::Domain> {
        responses
            .iter()
            .into_group_map_by(|res| &res.todo_id)
            .values()
            .map(|res_group| {
                let domain_tags: Vec<Tag> = res_group
                    .iter()
                    .map(|res| Tag::new(&res.tag_id, &res.tag_name, &res.tag_color))
                    .collect();
                let res = res_group[0];

                Todo::new(
                    &res.todo_id, &res.todo_name, res.todo_memo.as_ref().unwrap_or(&"".to_string()),
                    domain_tags
                )
            })
            .collect()
    }
}
