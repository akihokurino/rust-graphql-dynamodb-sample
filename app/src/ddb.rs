use aws_sdk_dynamodb::Client;
use std::marker::PhantomData;

pub mod stock;

pub struct Dao<T> {
    cli: Client,
    table_name_provider: TableNameProvider,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Dao<T> {
    pub async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);
        let table_name_provider = TableNameProvider::new();

        Self {
            cli: client,
            table_name_provider,
            _phantom: PhantomData,
        }
    }
}

pub struct TableNameProvider {
    pub prefix: String,
}

impl TableNameProvider {
    pub fn new() -> Self {
        Self {
            prefix: "".to_string(),
        }
    }

    pub fn with(&self, base_name: &str) -> String {
        format!("{}{}", self.prefix.as_str(), base_name)
    }
}
