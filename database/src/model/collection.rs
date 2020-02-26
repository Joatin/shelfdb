use chrono::{
    DateTime,
    Utc,
};
use std::mem;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Collection {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: Utc::now(),
        }
    }

    pub fn get_size(&self) -> usize {
        let mut size = mem::size_of::<Self>();
        if let Some(desc) = &self.description {
            size += mem::size_of_val(desc);
        }
        size
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_owned(),
            description: None,
            created_at: Utc::now(),
        }
    }
}
