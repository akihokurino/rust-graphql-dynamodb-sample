use crate::graph::inputs::CreateStockInput;
use crate::graph::models::Stock;
use crate::graph::Context;
use juniper::FieldResult;

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
