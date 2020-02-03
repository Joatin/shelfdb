use crate::server::admin::context::Context;
use juniper::{FieldResult, FieldError};
use std::sync::Arc;
use crate::server::util::make_sync;
use crate::server::admin::schema_type::SchemaType;


pub struct Query;

impl Query {
    pub fn new() -> Self {
        Self {}
    }
}

#[juniper::object(Context = Arc<Context>)]
impl Query {
    #[graphql(
    description = "Returns the current version of this API",
    )]
    fn api_version(context: &Arc<Context>) -> FieldResult<String> {
        Ok("1.0".to_owned())
    }

    #[graphql(
    description = "Returns all schemas stored in the database",
    )]
    fn schemas(context: &Arc<Context>) -> FieldResult<Vec<SchemaType>> {
        let context = Arc::clone(&context);

        let res = make_sync(async move {
            let logger = context.get_logger();
            context.db.schemas(&logger).await
        });

        match res {
            Ok(r) => {
                Ok(r.into_iter().map(|i| SchemaType::from(i)).collect())
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
}