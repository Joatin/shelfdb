use serde_json::Value;
use std::{
    collections::HashMap,
    mem,
};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Document {
    pub id: Uuid,
    pub fields: HashMap<String, Value>,
}

impl Document {
    pub fn get_size(&self) -> usize {
        let mut size = mem::size_of::<Self>();
        for (key, value) in &self.fields {
            size += mem::size_of_val(&key);
            size += mem::size_of_val(&value);
        }
        size
    }
}

impl Into<Value> for Document {
    fn into(self) -> Value {
        serde_json::to_value(&self).expect("This is always serializable")
    }
}
