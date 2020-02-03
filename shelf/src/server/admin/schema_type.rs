use crate::collection::Schema;
use uuid::Uuid;
use chrono::{Utc, DateTime};

#[derive(GraphQLObject)]
#[graphql(
    name = "Schema",
    description = "A schema in the database. A schema is a separate namespace, fully isolated from other schemas"
)]
pub struct SchemaType {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>
}

impl From<Schema> for SchemaType {
    fn from(value: Schema) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            created_at: value.created_at,
        }
    }
}