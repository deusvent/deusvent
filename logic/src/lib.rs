//! Main shared library which is used across all the clients and servers

#![deny(missing_docs)] // Logic is a main shared library - require docs for all public interfaces

pub mod auth;
pub mod date;
pub mod messages;
pub mod time;
