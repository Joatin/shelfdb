
#[macro_use] extern crate slog;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate juniper;
#[macro_use] extern crate failure;

mod collection;
mod server;
mod database;

use failure::Error;
use sloggers::terminal::TerminalLoggerBuilder;
use sloggers::types::Severity;
use sloggers::Build;
use crate::server::Server;
use crate::database::{Database, FileStore, MemoryCache};
use graceful::SignalGuard;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let signal_guard = SignalGuard::new();

    let logger = TerminalLoggerBuilder::new().level(Severity::Debug).build().unwrap();

    info!(logger, "Starting SHELF");
    info!(logger, "Running on: {} {}", sys_info::os_type().unwrap(), sys_info::os_release().unwrap());

    let store = FileStore::new(&logger).await?;
    let cache = MemoryCache::new(&logger).await?;
    let database = Database::new(&logger, store, cache).await?;
    let server = Server::start(&logger, database).await?;


    signal_guard.at_exit(move |_sig| {
        info!(logger, "Initiating shutdown...");
        server.stop();
        info!(logger, "Bye, Bye!");
    });

    Ok(())
}