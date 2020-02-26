use crate::{
    admin::{
        schema_input::SchemaInput,
        schema_type::SchemaType,
    },
    context::Context,
};
use juniper::FieldResult;
use shelf_database::{
    Cache,
    Schema,
    Store,
};
use std::marker::PhantomData;

pub struct Mutation<C: Cache, S: Store> {
    phantom_cache: PhantomData<C>,
    phantom_store: PhantomData<S>,
}

impl<C: Cache, S: Store> Mutation<C, S> {
    pub fn new() -> Self {
        Self {
            phantom_cache: PhantomData,
            phantom_store: PhantomData,
        }
    }
}

#[juniper::graphql_object(Context = Context<C, S>)]
impl<C: Cache, S: Store> Mutation<C, S> {
    async fn set_schema(context: &Context<C, S>, input: SchemaInput) -> FieldResult<SchemaType> {
        let schema = Schema::new(input.id, &input.name, input.description);

        let res = SchemaType::from(schema.clone());

        context
            .db
            .cache()
            .insert_schema(&context.logger, schema, "")
            .await?;

        Ok(res)
    }

    fn set_collection(
        _context: &Context<C, S>,
        _name: String,
        _schema_name: String,
    ) -> FieldResult<bool> {
        Ok(true)
    }

    fn set_document(
        _context: &Context<C, S>,
        _name: String,
        _collection_name: String,
        _schema_name: String,
    ) -> FieldResult<bool> {
        Ok(true)
    }
}
