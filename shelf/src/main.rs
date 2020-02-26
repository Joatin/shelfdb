#[macro_use]
extern crate slog;
#[macro_use]
extern crate human_panic;

use colored::*;
use failure::Error;
use graceful::SignalGuard;
use shelf_config::Config;
use shelf_database::Database;
use shelf_file_store::FileStore;
use shelf_memory_cache::MemoryCache;
use shelf_server::Server;
use sloggers::{
    terminal::TerminalLoggerBuilder,
    types::Severity,
    Build,
};
use std::{
    process,
    str::FromStr,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_panic!();
    let signal_guard = SignalGuard::new();

    let temp_log = TerminalLoggerBuilder::new()
        .level(Severity::Trace)
        .build()
        .unwrap();
    info!(temp_log, "🎉 Starting SHELF");
    match Config::load(&temp_log) {
        Ok(config) => {
            drop(temp_log);
            let logger = TerminalLoggerBuilder::new()
                .level(Severity::from_str(&config.log_level).expect("Got invalid log level"))
                .build()
                .unwrap();

            debug!(
                logger,
                "Running on: {} {}",
                sys_info::os_type().unwrap().yellow(),
                sys_info::os_release().unwrap().yellow()
            );
            let store = FileStore::new(&logger, &config).await?;
            let cache = MemoryCache::new(&logger).await?;
            let database = Database::new(&logger, &config, store, cache).await?;
            let server = Server::start(&logger, &config, database).await?;

            signal_guard.at_exit(move |_sig| {
                info!(logger, "{}", "↘️ Initiating shutdown...".cyan());
                server.stop();
                info!(logger, "👋 Bye, Bye!");
            });
        }
        Err(err) => {
            error!(temp_log, "Failed to load config"; "error" => format!("{}", err));
            drop(temp_log);
            // we want to exit with a non zero exit code, since the configuration was faulty
            process::exit(1);
        }
    }

    Ok(())
}
