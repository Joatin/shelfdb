#![allow(clippy::module_inception)]
#![deny(dead_code)]
#![deny(unused_imports)]

#[macro_use] extern crate slog;

mod file_store;

pub use self::file_store::FileStore;