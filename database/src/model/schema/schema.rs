use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use failure::Error;
use crate::{Store, Collection};
use std::collections::HashMap;
use graphql_parser::schema::{Document, Definition};
use graphql_parser::parse_schema;
use slog::Logger;
use crate::model::schema::validate_graphql_schema_correctness::validate_graphql_schema_correctness;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip)]
    pub collections: Vec<Collection>,
    graphql_schemas: HashMap<u32, String>
}

impl Schema {

    pub fn get_system_schema() -> Self {
        let mut schema = Self::default();
        schema.name = "system".to_owned();
        schema
    }

    pub fn get_default_schema() -> Self {
        let mut schema = Self::default();
        schema.name = "shelf".to_owned();
        schema
    }

    pub fn new(id: Uuid, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
            created_at: Utc::now(),
            collections: vec![],
            graphql_schemas: Self::default_graphql_schemas()
        }
    }

    fn default_graphql_schemas() -> HashMap<u32, String> {
        let mut graphql_schemas = HashMap::new();

        let data = include_str!("base_schema.graphql");

        graphql_schemas.insert(1, data.to_owned());

        graphql_schemas
    }

    pub fn definition(&self) -> Result<Document, Error> {
        match self.graphql_schemas.iter().last() {
            Some((index, schema)) => {
                let doc = parse_schema(schema)?;

                Ok(doc)
            },
            None => {
                bail!("No definition found")
            },
        }
    }

    pub fn validate_definition(&self, logger: &Logger) -> Result<(), Error> {
        let definition = self.definition()?;
        validate_graphql_schema_correctness(&logger, &definition)?;



        Ok(())
    }

    pub fn migrate_definition(&self) -> Result<(), Error> {
        unimplemented!()
    }
}

impl Default for Schema {
    fn default() -> Self {
        let graphql_schemas = Self::default_graphql_schemas();

        Self {
            id: Uuid::new_v4(),
            name: "".to_owned(),
            description: None,
            created_at: Utc::now(),
            collections: vec![],
            graphql_schemas
        }
    }
}