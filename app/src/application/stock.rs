use crate::ddb::Dao;
use crate::domain::stock::Stock;
use crate::AppResult;

pub struct Application {
    stock_dao: Dao<Stock>,
}

impl Application {
    pub async fn new() -> Self {
        let stock_dao: Dao<Stock> = Dao::new().await;
        Self { stock_dao }
    }

    pub async fn list(&self) -> AppResult<Vec<Stock>> {
        let stocks = self.stock_dao.get_all().await?;
        Ok(stocks)
    }

    pub async fn get(&self, id: String) -> AppResult<Stock> {
        let stock = self.stock_dao.get(id).await?;
        Ok(stock)
    }

    pub async fn create(&self, name: String) -> AppResult<Stock> {
        let stock = Stock::new(name);
        self.stock_dao.put(&stock).await?;
        Ok(stock)
    }

    pub async fn delete(&self, id: String) -> AppResult<()> {
        self.stock_dao.delete(id).await?;
        Ok(())
    }
}
