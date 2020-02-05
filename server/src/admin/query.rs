use crate::admin::context::Context;
use juniper::{FieldResult, FieldError};
use crate::util::make_sync;
use crate::admin::schema_type::SchemaType;


pub struct Query;

impl Query {
    pub fn new() -> Self {
        Self {}
    }
}

#[juniper::object(Context = Context)]
impl Query {
    #[graphql(
    description = "Returns the current version of this API",
    )]
    fn api_version() -> FieldResult<String> {
        Ok("1.0".to_owned())
    }

    #[graphql(
    description = "Returns all schemas stored in the database",
    )]
    fn schemas(context: &Context) -> FieldResult<Vec<SchemaType>> {
        let context = context.clone();
        let schemas = context.db.schemas();
        Ok(schemas.iter().map(|i| SchemaType::from(i)).collect())
    }
}