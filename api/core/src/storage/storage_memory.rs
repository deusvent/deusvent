//! Memory implementation of a Storage

use std::{
    collections::{BTreeMap, HashMap},
    pin::Pin,
    sync::Mutex,
};

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    operation::put_item::builders::PutItemFluentBuilder, types::AttributeValue, Client,
};
use futures::{stream, Stream};

use crate::entities::UserId;

use super::{Entity, Key, Storage, StorageErr};

/// Memory storage, used only for testing and development. In case of errors, it panics most of the time
/// to highlight mistakes early in the development process
pub struct MemoryStorage {
    data: Mutex<BTreeMap<String, PutItemFluentBuilder>>,
    client: Client,
}

impl Storage for MemoryStorage {
    async fn new(_: &'static str) -> Self {
        let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&shared_config);
        Self {
            data: Mutex::from(BTreeMap::default()),
            client,
        }
    }

    async fn write<T: Entity>(&self, entity: &T) -> Result<(), StorageErr> {
        let pk = entity.key().user_id.as_str().to_string();
        let sk = format!("{}_{}", T::entity_type(), entity.key().entity_id);
        let storage_key = format!("{}_{}", pk, sk);
        let mut data = self.data.lock().expect("Error locking data");
        data.insert(
            storage_key,
            entity.serialize(
                self.client
                    .put_item()
                    .item("pk", AttributeValue::S(pk))
                    .item("sk", AttributeValue::S(sk)),
            ),
        );
        Ok(())
    }

    async fn read<T>(&self, key: Key) -> Result<T, StorageErr>
    where
        T: Entity,
    {
        let pk = key.user_id.as_str().to_string();
        let sk = format!("{}_{}", T::entity_type(), key.entity_id);
        let storage_key = format!("{}_{}", pk, sk);
        let data = self.data.lock().expect("Error locking data");
        let item = match data.get(&storage_key) {
            Some(item) => item,
            None => return Err(StorageErr::NotFound),
        };
        let data = item.as_input().clone().build().unwrap().item.unwrap();
        T::deserialize(key, data)
    }

    async fn delete(
        &self,
        user_id: &UserId,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
    ) -> Result<usize, StorageErr> {
        let mut data = self.data.lock().expect("Error locking data");
        let len_before = data.len();
        let prefix = match (entity_type, entity_id) {
            (None, None) => format!("{}_", user_id.as_str()),
            (Some(entity), None) => format!("{}_{}_", user_id.as_str(), entity),
            (Some(entity), Some(sk)) => format!("{}_{}_{}", user_id.as_str(), entity, sk),
            (None, Some(_)) => {
                return Err(StorageErr::ValidationError(
                    "Cannot delete by sort key without entity name".to_string(),
                ))
            }
        };
        data.retain(|k, _| !k.starts_with(&prefix));
        Ok(len_before - data.len())
    }

    async fn find<T>(&self, user_id: &UserId) -> Pin<Box<dyn Stream<Item = Result<T, StorageErr>>>>
    where
        T: Entity + 'static,
    {
        let entity_type = T::entity_type();
        let prefix = format!("{}_{}", user_id.as_str(), entity_type);
        let data = self.data.lock().expect("Error locking data");
        let mut results = vec![];
        for (key, value) in data.range(prefix..) {
            if !key.starts_with(key) {
                break;
            }
            let data = value.as_input().clone().build().unwrap().item.unwrap();
            let full_sk = read_string_attribute("sk", &data);
            let entity_id = &full_sk[entity_type.len() + 1..]; // sort key is "[ENTITY_TYPE]_[ENTITY_ID]", remove prefix
            let key = Key {
                user_id: read_string_attribute("pk", &data)
                    .parse()
                    .expect("Expected valid user_id"),
                entity_id: entity_id.to_string(),
            };
            let entity = T::deserialize(key, data).expect("Deserialization should succeed");
            results.push(Ok(entity));
        }
        Box::pin(stream::iter(results))
    }
}

fn read_string_attribute(key: &str, data: &HashMap<String, AttributeValue>) -> String {
    data.get(key)
        .unwrap_or_else(|| panic!("{} attribute should exists", key))
        .as_s()
        .unwrap_or_else(|_| panic!("{} attribute should be of type string", key))
        .clone()
}
