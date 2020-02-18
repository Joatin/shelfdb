#![allow(clippy::module_inception)]
#![deny(dead_code)]
#![deny(unused_imports)]

#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;

mod config;

pub use self::config::Config;
