use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use graphql_parser::schema::Document;
use graphql_parser::parse_schema;
use crate::util::ExtractedData;
use crate::util::extract_graphql_schema;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub(crate) graphql_schemas: HashMap<u32, String>
}

impl Schema {
    pub fn new(id: Uuid, name: &str, description: Option<String>) -> Self {
        Self {
            id,
            name: name.to_string(),
            description,
            created_at: Utc::now(),
            graphql_schemas: HashMap::new()
        }
    }

    pub fn definition(&self) -> Option<Document> {
        self.current_migration_version().map(|version| {
            self.graphql_schemas.get(&version).map(|raw| {
                parse_schema(raw).expect("Schemas are always validated before they are added to the schema")
            })
        }).flatten()
    }

    pub fn types(&self) -> Option<ExtractedData> {
        self.definition().map(|d| extract_graphql_schema(&d))
    }

    pub fn current_migration_version(&self) -> Option<u32> {
        let mut highest = None;
        for i in self.graphql_schemas.keys() {
            if highest.is_none() || highest.unwrap() < *i {
                highest = Some(*i);
            }
        }
        highest
    }
}