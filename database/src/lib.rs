#![allow(clippy::module_inception)]
#![deny(dead_code)]
#![deny(unused_imports)]


#[macro_use] extern crate slog;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;

mod database;
mod store;
mod cache;
mod model;
pub(crate) mod util;
pub mod test;

pub use self::database::Database;
pub use self::store::Store;
pub use self::cache::*;
pub use self::model::*;