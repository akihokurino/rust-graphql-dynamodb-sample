use crate::graph::outputs::Stock;
use crate::graph::Context;
use crate::graph::FieldErrorWithCode;
use juniper::FieldResult;

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn stocks(context: &Context) -> FieldResult<Vec<Stock>> {
        let stocks = context
            .stock_application
            .list()
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(stocks
            .iter()
            .map(|v| Stock {
                id: v.id.to_owned(),
                name: v.name.to_owned(),
            })
            .collect())
    }

    async fn stock(context: &Context, id: String) -> FieldResult<Stock> {
        let stock = context
            .stock_application
            .get(id)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(Stock {
            id: stock.id.to_owned(),
            name: stock.name.to_owned(),
        })
    }
}
