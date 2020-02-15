use shelf_database::{Database, Cache, Store};
use slog::Logger;
use uuid::Uuid;
use std::sync::{Arc, RwLock};

pub struct Context<C: Cache, S: Store> {
    pub db: Arc<RwLock<Database<C, S>>>,
    pub logger: Logger,
}

impl<C: Cache, S: Store> Context<C, S> {
    pub fn new(logger: &Logger, db: Arc<RwLock<Database<C, S>>>) -> Self {
        Self {
            db,
            logger: logger.clone()
        }
    }

    pub fn new_request(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
            logger: self.logger.new(o!("request_id" => Uuid::new_v4().to_string()))
        }
    }
}

impl<C: Cache, S: Store> juniper::Context for Context<C, S> {}