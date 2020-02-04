
#[macro_use] extern crate slog;
#[macro_use] extern crate human_panic;

use failure::Error;
use sloggers::terminal::TerminalLoggerBuilder;
use sloggers::types::Severity;
use sloggers::Build;
use shelf_server::Server;
use shelf_database::{Database, FileStore, MemoryCache};
use graceful::SignalGuard;
use std::process;
use shelf_config::Config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_panic!();
    let signal_guard = SignalGuard::new();

    let logger = TerminalLoggerBuilder::new().level(Severity::Trace).build().unwrap();

    info!(logger, "Starting SHELF");
    info!(logger, "Running on: {} {}", sys_info::os_type().unwrap(), sys_info::os_release().unwrap());

    if let Ok(config) = Config::load(&logger) {
        let store = FileStore::new(&logger).await?;
        let cache = MemoryCache::new(&logger).await?;
        let database = Database::new(&logger, store, cache).await?;
        let server = Server::start(&logger, &config, database).await?;


        signal_guard.at_exit(move |_sig| {
            info!(logger, "Initiating shutdown...");
            server.stop();
            info!(logger, "Bye, Bye!");
        });
    } else {
        drop(logger);
        // we want to exit with a non zero exit code, since the configuration was faulty
        process::exit(1);
    }

    Ok(())
}