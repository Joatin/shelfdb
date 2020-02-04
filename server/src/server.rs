use failure::Error;
use slog::Logger;
use shelf_database::Database;
use std::sync::Arc;
use crate::admin::start_admin_server;
use crate::client::start_client_server;
use shelf_config::Config;


pub struct Server {
    admin_handle: Box<dyn FnOnce()>,
    client_handle: Box<dyn FnOnce()>,
}

impl Server {
    pub async fn start(logger: &Logger, config: &Config, db: Database) -> Result<Self, Error> {

        info!(logger, "Setting up servers");

        let arc_db = Arc::new(db);
        let admin_handle = start_admin_server(&logger, &config, Arc::clone(&arc_db))?;
        let client_handle = start_client_server(&logger, &config, arc_db)?;

        Ok(Self {
            admin_handle: Box::new(admin_handle),
            client_handle: Box::new(client_handle),
        })
    }

    pub fn stop(self) {
        (self.admin_handle)();
        (self.client_handle)();
    }
}




