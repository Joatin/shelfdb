use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt::Display;
use failure::_core::fmt::{Formatter, Error};
use crate::Store;
use std::collections::{LinkedList, HashMap};


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip)]
    pub documents: LinkedList<HashMap<String, String>>
}

impl Collection {

}

impl Default for Collection {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_owned(),
            description: None,
            created_at: Utc::now(),
            documents: LinkedList::new()
        }
    }
}