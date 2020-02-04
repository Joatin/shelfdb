use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt::Display;
use failure::_core::fmt::{Formatter, Error};
use crate::Store;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Schema {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>
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
            created_at: Utc::now()
        }
    }

    pub async fn save<S: Store + Send + Sync + 'static>(&self, _store: &S) {
        unimplemented!()
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_owned(),
            description: None,
            created_at: Utc::now()
        }
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "~~~ SCHEMA {} ~~~", self.name)?;
        writeln!(f, "ID:          {}", self.id)?;
        if self.description.is_some() {
            writeln!(f, "DESCRIPTION: {}", self.description.as_ref().unwrap())?;
        }
        writeln!(f, "CREATED:     {}", self.created_at)
    }
}