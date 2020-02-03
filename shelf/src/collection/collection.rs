use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt::Display;
use failure::_core::fmt::{Formatter, Error};


#[derive(Serialize, Deserialize, Clone)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>
}

impl Collection {

    pub fn get_schema_version_collection() -> Self {
        let mut collection = Self::default();
        collection.name = "schema_version".to_owned();
        collection
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_owned(),
            description: None,
            created_at: Utc::now()
        }
    }
}

impl Display for Collection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "~~~ COLLECTION {} ~~~", self.name)?;
        writeln!(f, "ID:          {}", self.id)?;
        if self.description.is_some() {
            writeln!(f, "DESCRIPTION: {}", self.description.as_ref().unwrap())?;
        }
        writeln!(f, "CREATED:     {}", self.created_at)
    }
}