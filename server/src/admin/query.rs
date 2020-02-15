use crate::context::Context;
use juniper::FieldResult;
use crate::admin::schema_type::SchemaType;
use std::marker::PhantomData;
use shelf_database::{Cache, Store};
use shelf_database::CacheSchema;


pub struct Query<C: Cache, S: Store> {
    phantom_cache: PhantomData<C>,
    phantom_store: PhantomData<S>
}

impl<C: Cache, S: Store> Query<C, S> {
    pub fn new() -> Self {
        Self {
            phantom_cache: PhantomData,
            phantom_store: PhantomData
        }

    }
}

#[juniper::object(Context = Context<C, S>)]
impl<C: Cache, S: Store> Query<C, S> {
    #[graphql(
    description = "Returns the current version of this API",
    )]
    fn api_version() -> FieldResult<String> {
        Ok("1.0".to_owned())
    }

    #[graphql(
    description = "Returns all schemas stored in the database",
    )]
    fn schemas(context: &Context<C, S>) -> FieldResult<Vec<SchemaType>> {
        let db = context.db.read().unwrap();
        Ok(db.schemas().iter().map(|i| {
            let lock = i.read().unwrap();
            SchemaType::from(lock.inner_schema())
        }).collect())
    }
}