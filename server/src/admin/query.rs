use crate::{
    admin::schema_type::SchemaType,
    context::Context,
};
use futures::StreamExt;
use juniper::FieldResult;
use shelf_database::{
    Cache,
    CacheSchema,
    Store,
};
use std::marker::PhantomData;

pub struct Query<C: Cache, S: Store> {
    phantom_cache: PhantomData<C>,
    phantom_store: PhantomData<S>,
}

impl<C: Cache, S: Store> Query<C, S> {
    pub fn new() -> Self {
        Self {
            phantom_cache: PhantomData,
            phantom_store: PhantomData,
        }
    }
}

#[juniper::graphql_object(Context = Context<C, S>)]
impl<C: Cache, S: Store> Query<C, S> {
    #[graphql(description = "Returns the current version of this API")]
    fn api_version() -> FieldResult<String> {
        Ok("1.0".to_owned())
    }

    #[graphql(description = "Returns all schemas stored in the database")]
    async fn schemas(context: &Context<C, S>) -> FieldResult<Vec<SchemaType>> {
        Ok(context
            .db
            .schemas()
            .then(|i| async move { SchemaType::from(i.inner_schema().await) })
            .collect()
            .await)
    }
}
