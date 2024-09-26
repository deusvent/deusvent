//! Main shared library which is used across all the clients and servers

#![deny(missing_docs)] // Logic is a main shared library - require docs for all public interfaces

pub mod datetime;
pub mod encryption;
pub mod messages;
pub mod server_error;

uniffi::setup_scaffolding!();
