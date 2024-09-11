//! Core services library for all API functionality

#![allow(async_fn_in_trait)] // Ignore it as we plan to use async trait only in our code which is fine
#![deny(missing_docs)]  // It's used by all API services and should be well documented

pub mod entities;
pub mod storage;
