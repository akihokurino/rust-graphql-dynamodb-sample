mod inputs;
mod models;

use crate::graph::inputs::*;
use crate::graph::models::*;
use juniper::{EmptySubscription, FieldResult, RootNode};

#[derive(Clone)]
pub struct Context {}

impl juniper::Context for Context {}

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    fn users(_context: &Context) -> FieldResult<Vec<User>> {
        let user = User {
            id: "1".to_string(),
            name: "田中太郎".to_string(),
        };
        Ok(vec![user])
    }

    fn user(_context: &Context, id: String) -> FieldResult<User> {
        let user = User {
            id: id.to_owned(),
            name: "田中太郎".to_string(),
        };
        Ok(user)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn create_user(_context: &Context, params: CreateUserInput) -> FieldResult<User> {
        let user = User {
            id: "1".to_string(),
            name: params.name.to_owned(),
        };
        Ok(user)
    }
}
