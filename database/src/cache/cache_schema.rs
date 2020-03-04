use crate::{
    util::{
        extract_graphql_schema,
        validate_graphql_schema_correctness,
    },
    CacheCollection,
    Collection,
    Schema,
};
use failure::Error;
use futures::{
    future::BoxFuture,
    stream::BoxStream,
    FutureExt,
};
use graphql_parser::parse_schema;
use slog::Logger;
use uuid::Uuid;

/// This trait wraps a regular schema, but lets us retrieve collections with
/// more convenience methods
pub trait CacheSchema: 'static + Send + Sync + Clone {
    type CacheCollection: CacheCollection;

    fn inner_schema(&self) -> BoxFuture<Schema>;
    fn set_schema(&self, schema: Schema) -> BoxFuture<()>;
    fn collections(&self) -> BoxStream<Self::CacheCollection>;
    fn insert_collection(&self, collection: Collection) -> BoxFuture<Result<(), Error>>;
    fn collection(&self, id: Uuid) -> BoxFuture<Option<Self::CacheCollection>>;
    fn collection_by_name<'a>(
        &'a self,
        name: &'a str,
    ) -> BoxFuture<'a, Option<Self::CacheCollection>>;

    fn validate<'a>(&'a self, logger: &'a Logger) -> BoxFuture<'a, Result<(), Error>> {
        async move {
            if let Some(definition) = self.inner_schema().await.definition() {
                validate_graphql_schema_correctness(&logger, &definition)?;
                // validate_against_current_collections(&logger,
                // &self.collections)?;
            }
            Ok(())
        }
        .boxed()
    }

    fn migrate<'a>(
        &'a self,
        logger: &'a Logger,
        new_graphql_schema: &'a str,
    ) -> BoxFuture<'a, Result<(), Error>> {
        async move {
            info!(logger, "Starting schema migration 🤓");
            let doc = parse_schema(new_graphql_schema)?;
            validate_graphql_schema_correctness(&logger, &doc)?;
            info!(logger, "New schema looks valid applying it to schema");
            let res = extract_graphql_schema(&doc);

            info!(
                logger,
                "Found {} collections and {} other types",
                res.collections.len(),
                res.other_types.len()
            );

            if res.collections.is_empty() {
                warn!(
                    logger,
                    "Since this collection does not have any collections it will be ignored!"
                );
                bail!("Schema has no collections");
            }

            match self.inner_schema().await.current_migration_version() {
                Some(current_version) => {
                    let mut inner_schema = self.inner_schema().await;
                    inner_schema
                        .graphql_schemas
                        .insert(current_version + 1, new_graphql_schema.to_string());
                    self.set_schema(inner_schema).await;

                    // TODO: Add and remove collections

                    // TODO: migrate data

                    info!(logger, "Done migrating schema");
                    Ok(())
                }
                None => {
                    info!(
                        logger,
                        "This schema has never has never been migrated, no data needs to be migrated"
                    );
                    let mut inner_schema = self.inner_schema().await;
                    inner_schema
                        .graphql_schemas
                        .insert(0, new_graphql_schema.to_string());
                    self.set_schema(inner_schema).await;

                    for coll in res.collections {
                        self.insert_collection(Collection::new(coll.name.to_string(), None)).await?;
                    }

                    info!(logger, "Done migrating schema");
                    Ok(())
                }
            }
        }.boxed()
    }
}
