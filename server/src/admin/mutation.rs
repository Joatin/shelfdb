use crate::admin::context::Context;
use juniper::{FieldResult, FieldError};
use crate::util::make_sync;
use shelf_database::Schema;
use crate::admin::schema_input::SchemaInput;
use crate::admin::schema_type::SchemaType;

pub struct Mutation;

impl Mutation {
    pub fn new() -> Self {
        Self {}
    }
}

#[juniper::object(Context = Context)]
impl Mutation {

    fn set_schema(context: &Context, input: SchemaInput) -> FieldResult<SchemaType> {
        let context = context.clone();

        let res = make_sync(async move {
            context.db.set_schema(&context.logger, Schema::new(
                input.id,
                input.name,
                input.description
            )).await?;

            context.db.schema(&context.logger, &input.id).await
        });

        match res {
            Ok(r) => {
                Ok(SchemaType::from(&r))
            },
            Err(err) => {
                let msg = format!("{}", err);
                Err(FieldError::new(
                    "Failed to fetch schemas",
                    graphql_value!({ "internal_error": msg })
                ))
            }
        }
    }

    fn set_collection(context: &Context, name: String, schema_name: String) -> FieldResult<bool> {
        Ok(true)
    }

    fn set_document(context: &Context, name: String, collection_name: String, schema_name: String) -> FieldResult<bool> {
        Ok(true)
    }
}