#![allow(clippy::module_inception)]
#![deny(dead_code)]
#![deny(unused_imports)]

#[macro_use]
extern crate slog;
#[macro_use]
extern crate failure;

mod memory_cache;
mod memory_cache_collection;
mod memory_cache_schema;

pub use self::memory_cache::MemoryCache;
