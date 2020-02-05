#[macro_use] extern crate slog;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;

mod file_store;

pub use self::file_store::FileStore;