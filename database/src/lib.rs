#![allow(clippy::module_inception)]
#![deny(dead_code)]
#![deny(unused_imports)]

#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

mod cache;
mod database;
mod model;
mod store;
pub mod test;
pub(crate) mod util;

pub use self::{
    cache::*,
    database::Database,
    model::*,
    store::Store,
};
