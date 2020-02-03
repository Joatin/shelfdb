use failure::Error;
use slog::Logger;
use crate::database::Database;
use std::sync::Arc;
use crate::server::admin::start_admin_server;
use crate::server::client::start_client_server;


pub struct Server {
    admin_handle: Box<dyn FnOnce()>,
    client_handle: Box<dyn FnOnce()>,
}

impl Server {
    pub async fn start(logger: &Logger, db: Database) -> Result<Self, Error> {

        info!(logger, "Setting up servers");

        let arc_db = Arc::new(db);
        let admin_handle = start_admin_server(&logger, Arc::clone(&arc_db), 3001)?;
        let client_handle = start_client_server(&logger, arc_db, 3000)?;

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




