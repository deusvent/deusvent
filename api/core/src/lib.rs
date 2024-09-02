#![allow(async_fn_in_trait)] // Ignore it as we plan to use async trait only in our code which is fine

pub mod entities;
pub mod storage;
pub mod storage_dynamodb;
pub mod storage_memory;
