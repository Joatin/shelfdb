#[macro_use] extern crate slog;
#[macro_use] extern crate failure;

mod memory_cache;

pub use self::memory_cache::MemoryCache;