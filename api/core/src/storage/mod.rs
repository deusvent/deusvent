//! Data storage - defines main "Storage" trait and DynamoDB/Memory implementation

pub mod storage_dynamodb;
pub mod storage_memory;

use std::{collections::HashMap, pin::Pin};

use aws_sdk_dynamodb::{
    operation::put_item::builders::PutItemFluentBuilder, types::AttributeValue,
};
use futures::Stream;

use crate::entities::UserId;

/// Storage error types
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum StorageErr {
    /// Data is invalid and cannot be written or read
    ValidationError(String),
    /// Request failed e.g. network error
    IOError(String),
    /// Requested entity not found
    NotFound,
}

/// Base user entity where partition key is user id
pub trait Entity {
    /// Return entity type name which is used as a static prefix for the sort key
    fn entity_type() -> &'static str;

    /// Return entity key
    fn key(&self) -> &Key;

    /// Serialize entity data to the writer for storing it in the database
    fn serialize(&self, writer: PutItemFluentBuilder) -> PutItemFluentBuilder;

    /// Deserialize an entity from the data read from the database
    fn deserialize(key: Key, data: HashMap<String, AttributeValue>) -> Result<Self, StorageErr>
    where
        Self: Sized;
}

/// Entity key - user_id as partition key and entity_id as a sort key
#[derive(Debug, PartialEq, Clone)]
pub struct Key {
    // TODO Remove public fields from here as well
    /// User identifier
    pub user_id: UserId,
    /// Entity identifier
    pub entity_id: String,
}

/// Base trait for the storage provider
pub trait Storage {
    /// Creates a new Storage for the given table name
    async fn new(table: &'static str) -> Self;

    /// Store user entity in the database
    async fn write<T: Entity>(&self, entity: &T) -> Result<(), StorageErr>;

    /// Read user entity from the database
    async fn read<T>(&self, key: Key) -> Result<T, StorageErr>
    where
        T: Entity;

    /// Find all the entities for the given partition key and entity type, output is streamed
    async fn find<T>(&self, user_id: &UserId) -> Pin<Box<dyn Stream<Item = Result<T, StorageErr>>>>
    where
        T: Entity + 'static;

    /// Delete entities in the give partition with optional entity type and sort key
    async fn delete(
        &self,
        user_id: &UserId,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
    ) -> Result<usize, StorageErr>;

    /// Delete all the entries for the giver user id
    async fn delete_user_data(&self, user_id: &UserId) -> Result<usize, StorageErr> {
        self.delete(user_id, None, None).await
    }

    /// Delete all the entries of the given type for the given user
    async fn delete_entities(
        &self,
        user_id: &UserId,
        entity_type: &str,
    ) -> Result<usize, StorageErr> {
        self.delete(user_id, Some(entity_type), None).await
    }

    /// Delete an entity
    async fn delete_entity<T>(&self, entity: T) -> Result<usize, StorageErr>
    where
        T: Entity,
    {
        self.delete(
            &entity.key().user_id,
            Some(T::entity_type()),
            Some(&entity.key().entity_id),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use lazy_static::lazy_static;
    use time::OffsetDateTime;

    use crate::{
        entities::{Account, UserId},
        storage::{storage_dynamodb::DynamoStorage, storage_memory::MemoryStorage},
    };

    use super::*;

    lazy_static! {
        static ref User1: UserId = "01D39ZY06FGSCTVN4T2V9PKHFZ".parse().unwrap();
        static ref User2: UserId = "2D9RW50MA499CMAGHM6DD42DTP".parse().unwrap();
    }

    const TABLE: &str = "game_data_test";

    fn random_account(user: &UserId) -> Account {
        Account {
            key: Key {
                user_id: user.clone(),
                entity_id: Account::entity_type().to_string(),
            },
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
        }
    }

    async fn test_storage(storage: &impl Storage) {
        // Read/Write/Overwrite
        let acc_orig = random_account(&User1);
        storage.write(&acc_orig).await.unwrap();
        let mut acc_read = storage.read(acc_orig.key.clone()).await.unwrap();
        assert_eq!(acc_orig, acc_read);
        acc_read.created_at += 1;
        storage.write(&acc_read).await.unwrap();
        let acc_modified = storage.read(acc_read.key.clone()).await.unwrap();
        assert_ne!(acc_orig, acc_modified);
        assert_eq!(acc_modified.created_at, acc_read.created_at);

        // Delete/Read
        let key = acc_orig.key.clone();
        storage.delete_entity(acc_orig).await.unwrap();
        assert!(matches!(
            storage.read::<Account>(key).await,
            Err(StorageErr::NotFound)
        ));

        // Find
        let acc = random_account(&User1);
        storage.write(&acc).await.unwrap();
        let found = storage
            .find::<Account>(&User1)
            .await
            .map(|v| v.unwrap())
            .collect::<Vec<_>>()
            .await;
        assert_eq!(found, vec![acc]);
        assert!(storage.find::<Account>(&User2).await.next().await.is_none());

        // Delete entities
        storage.write(&random_account(&User2)).await.unwrap();
        storage
            .delete_entities(&User2, Account::entity_type())
            .await
            .unwrap();
        assert!(storage.find::<Account>(&User2).await.next().await.is_none());
    }

    async fn cleanup(storage: &impl Storage) {
        storage.delete_user_data(&User1).await.unwrap();
        storage.delete_user_data(&User2).await.unwrap();
        assert!(storage.find::<Account>(&User1).await.next().await.is_none());
        assert!(storage.find::<Account>(&User2).await.next().await.is_none());
    }

    #[tokio::test]
    async fn test_memory_storage() {
        let storage = MemoryStorage::new(TABLE).await;
        cleanup(&storage).await;
        test_storage(&storage).await;
        cleanup(&storage).await;
    }

    #[tokio::test]
    async fn test_dynamodb_storage() {
        if std::env::var("AWS_PROFILE").is_err() && std::env::var("AWS_ACCESS_KEY_ID").is_err() {
            println!("No AWS credentials set, skipping DynamoDB test");
            return;
        }
        let storage = DynamoStorage::new(TABLE).await;
        cleanup(&storage).await;
        test_storage(&storage).await;
        cleanup(&storage).await;
    }
}
