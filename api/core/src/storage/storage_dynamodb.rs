//! DynamoDB based storage

use std::{collections::HashMap, pin::Pin};

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    error::DisplayErrorContext,
    types::{AttributeValue, DeleteRequest, Select, WriteRequest},
    Client,
};
use futures::Stream;

use async_stream::stream;

use crate::entities::UserId;

use super::{Entity, Key, Storage, StorageErr};

/// DynamoDB based storage:
/// pk - partition key which is user id
/// sk - compound sort key of a form [ENTITY_TYPE][ENTITY_ID]
pub struct DynamoStorage {
    client: Client,
    table: &'static str,
}

impl DynamoStorage {
    async fn batch_delete(&self, data: Vec<(String, String)>) -> Result<(), StorageErr> {
        self.client
            .batch_write_item()
            .request_items(
                self.table,
                data.into_iter()
                    .map(|(pk, sk)| {
                        WriteRequest::builder()
                            .set_delete_request(Some(
                                DeleteRequest::builder()
                                    .key("pk", AttributeValue::S(pk))
                                    .key("sk", AttributeValue::S(sk))
                                    .build()
                                    .expect("DeleteRequest should be always created"),
                            ))
                            .build()
                    })
                    .collect(),
            )
            .send()
            .await
            .map_err(|err| {
                StorageErr::IOError(format!(
                    "Failed to delete the keys: {}",
                    DisplayErrorContext(&err)
                ))
            })
            .map(|_| ())
    }
}

impl Storage for DynamoStorage {
    async fn new(table: &'static str) -> Self {
        let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&shared_config);
        Self { client, table }
    }

    async fn write<T: Entity>(&self, entity: &T) -> Result<(), StorageErr> {
        let pk = AttributeValue::S(entity.key().user_id.as_str().to_string());
        let sk = sort_key(T::entity_type(), &entity.key().entity_id);
        let builder = self
            .client
            .put_item()
            .table_name(self.table)
            .item("pk", pk)
            .item("sk", sk);

        entity
            .serialize(builder)
            .send()
            .await
            .map(|_| ())
            .map_err(|err| {
                StorageErr::IOError(format!(
                    "Failed to write an entity: {}",
                    DisplayErrorContext(&err)
                ))
            })
    }

    async fn read<T>(&self, key: Key) -> Result<T, StorageErr>
    where
        T: Entity,
    {
        let pk = key.user_id.as_str().to_string();
        let sk = sort_key(T::entity_type(), key.entity_id.as_str());
        let keys = HashMap::from([
            ("pk".to_string(), AttributeValue::S(pk)),
            ("sk".to_string(), sk),
        ]);
        let data = self
            .client
            .get_item()
            .table_name(self.table)
            .set_key(Some(keys))
            .send()
            .await
            .map_err(|err| {
                StorageErr::IOError(format!(
                    "Failed to read an entity: {}",
                    DisplayErrorContext(&err)
                ))
            })?
            .item
            .ok_or(StorageErr::NotFound)?;
        T::deserialize(key, data)
    }

    async fn delete(
        &self,
        user_id: &UserId,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
    ) -> Result<usize, StorageErr> {
        let pk = user_id.as_str().to_string();
        // Delete a concrete entity by pk, entity name and sk
        if let (Some(name), Some(sk)) = (entity_type, entity_id) {
            return self
                .batch_delete(vec![(pk, format!("{}_{}", name, sk))])
                .await
                .map(|_| 1);
        }

        // DynamoDB doesn't provide a simple way to delete records by partition or by sort key prefix
        // Search those records first and then batch delete it
        let mut attribute_values =
            HashMap::from([(":pk".to_string(), AttributeValue::S(pk.to_string()))]);

        let filter = match entity_type {
            Some(name) => {
                attribute_values.insert(":sk".to_string(), AttributeValue::S(format!("{}_", name)));
                "pk = :pk AND sk > :sk"
            }
            None => "pk = :pk",
        };

        let mut items = self
            .client
            .query()
            .table_name(self.table)
            .key_condition_expression(filter)
            .set_expression_attribute_values(Some(attribute_values))
            .select(Select::SpecificAttributes)
            .projection_expression("pk,sk")
            .into_paginator()
            .items()
            .send();
        let mut deleted = 0;
        let mut delete_chunk = Vec::new();
        while let Some(v) = items.next().await {
            let data = v.map_err(|err| {
                StorageErr::IOError(format!(
                    "Failed to fetch keys for deletion: {}",
                    DisplayErrorContext(&err)
                ))
            })?;

            if delete_chunk.len() == 25 {
                deleted += delete_chunk.len();
                self.batch_delete(delete_chunk).await?;
                delete_chunk = vec![];
            }

            let pk = data.get("pk").expect("pk should exists");
            let pk = pk.as_s().expect("pk should be a string").to_owned();
            let sk = data.get("sk").expect("sk should exists");
            let sk = sk.as_s().expect("sk should be a string").to_owned();
            delete_chunk.push((pk, sk))
        }
        if !delete_chunk.is_empty() {
            deleted += delete_chunk.len();
            self.batch_delete(delete_chunk).await?;
        }
        Ok(deleted)
    }

    async fn find<T>(&self, user_id: &UserId) -> Pin<Box<dyn Stream<Item = Result<T, StorageErr>>>>
    where
        T: Entity + 'static,
    {
        let entity_type = T::entity_type();
        let pk = user_id.as_str().to_string();
        let mut attributes = HashMap::new();
        attributes.insert(":pk".to_string(), AttributeValue::S(pk));
        attributes.insert(
            ":sk".to_string(),
            AttributeValue::S(format!("{}_", entity_type)),
        );

        let res = self
            .client
            .query()
            .table_name(self.table)
            .key_condition_expression("pk = :pk AND sk > :sk")
            .set_expression_attribute_values(Some(attributes));
        let mut paginator = res.into_paginator().items().send();
        let stream = stream! {
            while let Some(v) = paginator.next().await {
                let data = v.map_err(|err| StorageErr::IOError(format!("Error streaming entity: {}", DisplayErrorContext(&err))))?;
                let user_id = read_string_attribute("pk", &data)?;
                let full_sk = read_string_attribute("sk", &data)?;
                let entity_id = &full_sk[entity_type.len() + 1..]; // sort key is "[ENTITY_TYPE]_[ENTITY_ID]", remove prefix
                let key = Key {
                     user_id: user_id.parse().map_err(|_| StorageErr::ValidationError("Cannot create user id".to_string()))?,
                     entity_id: entity_id.to_string(),
                };
                yield T::deserialize(key, data);
            }
        };
        Box::pin(stream)
    }
}

fn sort_key(entity_name: &str, sk: &str) -> AttributeValue {
    AttributeValue::S(format!("{}_{}", entity_name, sk))
}

fn read_string_attribute(
    key: &str,
    data: &HashMap<String, AttributeValue>,
) -> Result<String, StorageErr> {
    data.get(key)
        .ok_or(StorageErr::ValidationError(format!(
            "{} attribute should exists",
            key
        )))
        .and_then(|v| {
            v.as_s().map_err(|_| {
                StorageErr::ValidationError(format!("{} attribute should be of type string", key))
            })
        })
        .cloned()
}
