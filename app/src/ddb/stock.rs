use crate::ddb::Dao;
use crate::domain::stock::Stock;
use crate::domain::*;
use crate::{AppError, AppResult};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use std::convert::TryFrom;

const TABLE_NAME: &str = "sample-stock";
const KEY_ID: &str = "ID";
const KEY_NAME: &str = "Name";

pub struct Entity {
    pub id: String,
    pub name: String,
}

// serdeなどシリアライズできるライブラリはまだないらしい
impl Entity {
    fn deserialize(data: HashMap<String, AttributeValue>) -> Option<Self> {
        if let (Some(AttributeValue::S(id)), Some(AttributeValue::S(name))) =
            (data.get(KEY_ID), data.get(KEY_NAME))
        {
            return Some(Entity {
                id: id.to_owned(),
                name: name.to_owned(),
            });
        }
        None
    }

    async fn save(&self, cli: &Client) -> AppResult<()> {
        cli.put_item()
            .table_name(TABLE_NAME)
            .item(KEY_ID, AttributeValue::S(self.id.clone()))
            .item(KEY_NAME, AttributeValue::S(self.name.clone()))
            .send()
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    fn primary_key(id: String) -> AttributeValue {
        AttributeValue::S(id.to_owned())
    }
}

impl TryFrom<&Entity> for stock::Stock {
    type Error = String;

    fn try_from(e: &Entity) -> Result<Self, Self::Error> {
        Ok(stock::Stock {
            id: e.id.to_owned(),
            name: e.name.to_owned(),
        })
    }
}

impl From<&stock::Stock> for Entity {
    fn from(d: &stock::Stock) -> Entity {
        Entity {
            id: d.id.to_owned(),
            name: d.name.to_owned(),
        }
    }
}

impl Dao<stock::Stock> {
    pub async fn get_all(&self) -> AppResult<Vec<Stock>> {
        let res = self
            .cli
            .scan()
            .table_name(TABLE_NAME)
            .send()
            .await
            .map_err(AppError::from)?;

        let mut entities: Vec<Entity> = vec![];
        for item in res.items.unwrap_or_default() {
            entities.push(Entity::deserialize(item).unwrap())
        }

        Ok(entities
            .iter()
            .map(|v| Stock::try_from(v).unwrap())
            .collect())
    }

    pub async fn get(&self, id: String) -> AppResult<Stock> {
        let res = self
            .cli
            .get_item()
            .table_name(TABLE_NAME)
            .key(KEY_ID, Entity::primary_key(id))
            .send()
            .await
            .map_err(AppError::from)?;
        let data = res.item.unwrap();

        Ok(Stock::try_from(&Entity::deserialize(data).unwrap()).unwrap())
    }

    pub async fn put(&self, item: &stock::Stock) -> AppResult<()> {
        let e: Entity = item.clone().into();
        e.save(&self.cli).await?;
        Ok(())
    }

    pub async fn delete(&self, id: String) -> AppResult<()> {
        self.cli
            .delete_item()
            .table_name(TABLE_NAME)
            .key(KEY_ID, Entity::primary_key(id))
            .send()
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
