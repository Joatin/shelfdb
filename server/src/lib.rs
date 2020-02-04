#[macro_use] extern crate slog;
#[macro_use] extern crate juniper;

mod admin;
mod client;
mod util;

mod server;

pub use self::server::Server;