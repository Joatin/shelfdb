#![allow(clippy::module_inception)]
#![deny(dead_code)]
#![deny(unused_imports)]

#[macro_use] extern crate slog;
#[macro_use] extern crate juniper;

mod admin;
mod client;
mod util;
mod context;

mod server;

pub use self::server::Server;