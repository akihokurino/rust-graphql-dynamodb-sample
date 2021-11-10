use crate::graph::models::Stock;
use crate::graph::Context;
use crate::graph::FieldErrorWithCode;
use juniper::FieldResult;

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
