use crate::ddb::Dao;
use crate::domain::stock::Stock;
use crate::AppResult;
use aws_sdk_dynamodb::Client;

#[derive(Clone)]
pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn list(&self) -> AppResult<Vec<Stock>> {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);

        let dao: Dao<Stock> = Dao::new();
        let stocks = dao.get_all(&client).await?;

        Ok(stocks)
    }

    pub async fn get(&self, id: String) -> AppResult<Stock> {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);

        let dao: Dao<Stock> = Dao::new();
        let stock = dao.get(&client, id).await?;

        Ok(stock)
    }

    pub async fn create(&self, name: String) -> AppResult<Stock> {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);

        let stock = Stock::new(name);

        let dao: Dao<Stock> = Dao::new();
        dao.put(&client, &stock).await?;

        Ok(stock)
    }

    pub async fn delete(&self, id: String) -> AppResult<()> {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);

        let dao: Dao<Stock> = Dao::new();
        dao.delete(&client, id).await?;

        Ok(())
    }
}
