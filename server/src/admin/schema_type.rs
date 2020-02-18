use chrono::{DateTime, Utc};
use shelf_database::Schema;
use uuid::Uuid;

#[derive(GraphQLObject)]
#[graphql(
    name = "Schema",
    description = "A schema in the database. A schema is a separate namespace, fully isolated from other schemas"
)]
pub struct SchemaType {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
}

impl From<&Schema> for SchemaType {
    fn from(value: &Schema) -> Self {
        Self {
            id: value.id,
            name: value.name.to_owned(),
            description: value.description.to_owned(),
            created_at: value.created_at,
        }
    }
}
