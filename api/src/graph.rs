mod inputs;
mod models;

use crate::graph::inputs::*;
use crate::graph::models::*;
use app::application::*;
use app::AppError;
use convert_case::{Case, Casing};
use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};
use strum_macros::Display as StrumDisplay;

#[derive(Clone)]
pub struct Context {
    pub stock_application: stock::Application,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Self {
        let stock_application = stock::Application::new();

        Self { stock_application }
    }
}

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    fn stocks(context: &Context) -> FieldResult<Vec<Stock>> {
        let stocks = context
            .stock_application
            .list()
            .map_err(FieldErrorWithCode::from)?;

        Ok(stocks
            .iter()
            .map(|v| Stock {
                id: v.id.to_owned(),
            })
            .collect())
    }

    fn stock(_context: &Context, id: String) -> FieldResult<Stock> {
        let stock = Stock { id: id.to_owned() };
        Ok(stock)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn create_stock(_context: &Context, _input: CreateStockInput) -> FieldResult<Stock> {
        let stock = Stock {
            id: "1".to_string(),
        };
        Ok(stock)
    }
}

#[derive(StrumDisplay, Debug)]
pub enum FieldErrorCode {
    BadRequest,
    UnAuthenticate,
    NotFound,
    Forbidden,
    Internal,
}

pub struct FieldErrorWithCode {
    err: AppError,
    code: FieldErrorCode,
}

impl From<AppError> for FieldErrorWithCode {
    fn from(err: AppError) -> Self {
        FieldErrorWithCode {
            err: err.clone(),
            code: match err {
                AppError::BadRequest(_) => FieldErrorCode::BadRequest,
                AppError::UnAuthenticate => FieldErrorCode::UnAuthenticate,
                AppError::Forbidden => FieldErrorCode::Forbidden,
                AppError::NotFound => FieldErrorCode::NotFound,
                AppError::Internal(_) => FieldErrorCode::Internal,
            },
        }
    }
}

impl From<FieldErrorWithCode> for FieldError {
    fn from(v: FieldErrorWithCode) -> Self {
        let code = v.code.to_string().to_case(Case::UpperSnake);

        FieldError::new(
            v.err,
            graphql_value!({
                "code": code,
            }),
        )
    }
}
