use shelf_database::Database;
use slog::Logger;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub db: Database,
    pub logger: Logger,
}

impl Context {
    pub fn new(logger: &Logger, db: Database) -> Self {
        Self {
            db,
            logger: logger.clone()
        }
    }

    pub fn new_request(&self) -> Self {
        Self {
            db: self.db.clone(),
            logger: self.logger.new(o!("request_id" => Uuid::new_v4().to_string()))
        }
    }
}

impl juniper::Context for Context {}
