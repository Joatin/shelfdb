use crate::database::Database;
use slog::Logger;
use uuid::Uuid;
use std::sync::Arc;

pub struct Context {
    pub db: Arc<Database>,
    logger: Logger,
}

impl Context {
    pub fn new(logger: &Logger, db: Arc<Database>) -> Self {
        Self {
            db,
            logger: logger.clone()
        }
    }

    pub fn get_logger(&self) -> Logger {
        self.logger.new(o!("request_id" => Uuid::new_v4().to_string()))
    }
}

impl juniper::Context for Context {}