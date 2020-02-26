use chrono::{
    DateTime,
    Utc,
};
use juniper::FieldResult;
use shelf_database::Schema;
use uuid::Uuid;

//#[graphql(
//    name = "Schema",
//    description = "A schema in the database. A schema is a separate namespace,
// fully isolated from other schemas"
//)]
pub struct SchemaType {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
}

#[juniper::graphql_object]
impl SchemaType {
    fn id(&self) -> FieldResult<&Uuid> {
        Ok(&self.id)
    }

    fn name(&self) -> FieldResult<&String> {
        Ok(&self.name)
    }

    fn description(&self) -> FieldResult<&Option<String>> {
        Ok(&self.description)
    }

    fn created_at(&self) -> FieldResult<&DateTime<Utc>> {
        Ok(&self.created_at)
    }
}

impl From<Schema> for SchemaType {
    fn from(value: Schema) -> Self {
        Self {
            id: value.id,
            name: value.name.to_owned(),
            description: value.description.to_owned(),
            created_at: value.created_at,
        }
    }
}
