use aws_sdk_dynamodb::Client;
use std::marker::PhantomData;

pub mod stock;

#[derive(Clone)]
pub struct Dao<T> {
    cli: Client,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Dao<T> {
    pub async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);

        Self {
            cli: client,
            _phantom: PhantomData,
        }
    }
}
