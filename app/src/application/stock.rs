use crate::domain::stock::Stock;
use crate::AppResult;

#[derive(Clone)]
pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub fn list(&self) -> AppResult<Vec<Stock>> {
        Ok(vec![Stock::new()])
    }
}
