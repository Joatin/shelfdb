#![allow(clippy::module_inception)]
#![deny(dead_code)]
#![deny(unused_imports)]

#[macro_use]
extern crate slog;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate failure;

mod admin;
mod client;
mod context;
mod util;

mod server;

pub use self::server::Server;
