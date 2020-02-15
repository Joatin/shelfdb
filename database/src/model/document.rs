use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;


#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Document {
    pub id: Uuid,
    pub fields: HashMap<String, Value>
}