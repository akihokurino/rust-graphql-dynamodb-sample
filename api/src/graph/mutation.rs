use crate::graph::inputs::CreateStockInput;
use crate::graph::models::Stock;
use crate::graph::Context;
use crate::graph::FieldErrorWithCode;
use juniper::FieldResult;

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_stock(context: &Context, input: CreateStockInput) -> FieldResult<Stock> {
        let stock = context
            .stock_application
            .create(input.name)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(Stock {
            id: stock.id.to_owned(),
            name: stock.name.to_owned(),
        })
    }

    async fn delete_stock(context: &Context, id: String) -> FieldResult<bool> {
        context
            .stock_application
            .delete(id)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(true)
    }
}
