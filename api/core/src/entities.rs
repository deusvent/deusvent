//! Entities

use std::{collections::HashMap, str::FromStr};

use aws_sdk_dynamodb::{
    operation::put_item::builders::PutItemFluentBuilder, types::AttributeValue,
};
use logic::time::ServerTimestamp;
use ulid::Ulid;

use crate::storage::{Entity, Key, StorageErr};

/// User identifier, randomly generated ULID
#[derive(Debug, PartialEq, Clone)]
pub struct UserId(Ulid);

impl UserId {
    /// Creates new randomly generated user identifier
    pub fn generate() -> Self {
        Self(ulid::Ulid::new())
    }

    /// Returns string representation of a user_id
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for UserId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ulid::Ulid::from_string(s)
            .map(UserId)
            .map_err(|err| err.to_string())
    }
}

/// Player account
#[derive(Debug, PartialEq)]
pub struct Account {
    // TODO Having public fields is handy during development, but should be removed once we know access patters that we need
    /// Account key
    pub key: Key,
    /// Timestamp when account was created
    pub created_at: ServerTimestamp,
}

impl Entity for Account {
    fn entity_type() -> &'static str {
        "account"
    }

    fn key(&self) -> &Key {
        &self.key
    }

    fn serialize(&self, writer: PutItemFluentBuilder) -> PutItemFluentBuilder {
        writer.item("created_at", AttributeValue::N(self.created_at.as_string()))
    }

    fn deserialize(key: Key, data: HashMap<String, AttributeValue>) -> Result<Self, StorageErr> {
        let created_at = read_number_attribute(data, "created_at")?;
        Ok(Self { key, created_at })
    }
}

impl Account {
    /// Generate new account with random user_id
    pub fn generate() -> Self {
        let user_id = UserId::generate();
        // For account entity id is the same as a user id
        let entity_id = user_id.as_str().to_string();
        Self {
            created_at: ServerTimestamp::now(),
            key: Key { user_id, entity_id },
        }
    }
}

fn read_number_attribute<T: FromStr>(
    attributes: HashMap<String, AttributeValue>,
    key: &str,
) -> Result<T, StorageErr> {
    attributes
        .get(key)
        .ok_or_else(|| StorageErr::ValidationError(format!("{} attribute not found", key)))?
        .as_n()
        .map_err(|_| StorageErr::ValidationError(format!("{} is not a number attribute", key)))?
        .parse()
        .map_err(|_| StorageErr::ValidationError(format!("{} cannot be parsed as number", key)))
}
