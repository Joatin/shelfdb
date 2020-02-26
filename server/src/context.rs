use shelf_database::{
    Cache,
    Database,
    Store,
};
use slog::Logger;
use std::sync::Arc;
use uuid::Uuid;

pub struct Context<C: Cache, S: Store> {
    pub db: Arc<Database<C, S>>,
    pub logger: Logger,
}

impl<C: Cache, S: Store> Context<C, S> {
    pub fn new(logger: &Logger, db: Arc<Database<C, S>>) -> Self {
        Self {
            db,
            logger: logger.clone(),
        }
    }

    pub fn new_request(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
            logger: self
                .logger
                .new(o!("request_id" => Uuid::new_v4().to_string())),
        }
    }
}

impl<C: Cache, S: Store> juniper::Context for Context<C, S> {}
