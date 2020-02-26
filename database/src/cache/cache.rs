use crate::{
    CacheSchema,
    Schema,
    Store,
};
use failure::Error;
use futures::{
    future::BoxFuture,
    stream::BoxStream,
};
use slog::Logger;
use tokio::sync::broadcast::Receiver;
use uuid::Uuid;

/// A cache is mainly responsible for keeping all the indexes of objects
///
/// All methods except those that includes some sort of io should be strictly
/// synchronous. Even though async is nice, it does come with performance costs
pub trait Cache: Send + Sync + 'static {
    type CacheSchema: CacheSchema;

    fn load<'a, S: Store>(
        &'a self,
        logger: &'a Logger,
        store: &'a S,
    ) -> BoxFuture<Result<(), Error>>;
    fn save<'a, S: Store>(
        &'a self,
        logger: &'a Logger,
        store: &'a S,
    ) -> BoxFuture<Result<(), Error>>;

    /// Retrieves all schemas
    fn schemas(&self) -> BoxStream<Self::CacheSchema>;

    /// Retrieves a schema by it's id
    fn schema(&self, id: Uuid) -> BoxFuture<Option<Self::CacheSchema>>;

    /// Retrieves a schema by it's name
    fn schema_by_name<'a>(&'a self, name: &'a str) -> BoxFuture<'a, Option<Self::CacheSchema>>;

    /// Adds or replaces a new schema to the cache
    fn insert_schema<'a>(
        &'a self,
        logger: &'a Logger,
        schema: Schema,
        new_graphql_schema: &'a str,
    ) -> BoxFuture<'a, Result<(), Error>>;

    /// Gets the current size in bytes from the cache
    fn cache_size(&self) -> BoxFuture<usize>;

    /// Tells if this cache is empty
    fn is_empty(&self) -> BoxFuture<bool>;

    fn on_schema_updates(&self) -> Receiver<()>;
}
