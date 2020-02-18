use crate::util::extract_graphql_schema;
use crate::util::validate_graphql_schema_correctness;
use crate::{CacheCollection, Collection, Schema};
use failure::Error;
use graphql_parser::parse_schema;
use slog::Logger;
use std::ops::Deref;
use std::sync::RwLock;
use uuid::Uuid;

/// This trait wraps a regular schema, but lets us retrieve collections with more convenience
/// methods
pub trait CacheSchema: 'static + Send + Sync {
    type CacheCollection: CacheCollection;

    fn inner_schema(&self) -> &Schema;
    fn inner_schema_mut(&mut self) -> &mut Schema;
    fn collections(&self) -> &[RwLock<Self::CacheCollection>];
    fn set_collection(&mut self, collection: Collection) -> Result<(), Error>;
    fn collection(&self, id: Uuid) -> Option<&RwLock<Self::CacheCollection>>;
    fn collection_by_name(&self, name: &str) -> Option<&RwLock<Self::CacheCollection>>;

    fn validate(&self, logger: &Logger) -> Result<(), Error> {
        if let Some(definition) = self.inner_schema().definition() {
            validate_graphql_schema_correctness(&logger, &definition)?;
            // validate_against_current_collections(&logger, &self.collections)?;
        }
        Ok(())
    }

    fn migrate(&mut self, logger: &Logger, new_graphql_schema: &str) -> Result<(), Error> {
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

        match self.inner_schema().current_migration_version() {
            Some(current_version) => {
                self.inner_schema_mut()
                    .graphql_schemas
                    .insert(current_version + 1, new_graphql_schema.to_string());

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
                self.inner_schema_mut()
                    .graphql_schemas
                    .insert(0, new_graphql_schema.to_string());

                for coll in res.collections {
                    self.set_collection(Collection::new(coll.name.to_string(), None))?;
                }

                info!(logger, "Done migrating schema");
                Ok(())
            }
        }
    }
}

impl<C: CacheCollection> Deref for dyn CacheSchema<CacheCollection = C> {
    type Target = Schema;

    fn deref(&self) -> &Self::Target {
        &self.inner_schema()
    }
}
