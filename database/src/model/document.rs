use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;
use std::mem;


#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Document {
    pub id: Uuid,
    pub fields: HashMap<String, Value>
}


impl Document {
    pub fn get_size(&self) -> usize {
        let mut size = mem::size_of::<Self>();
        for (key, value) in &self.fields {
            size +=  mem::size_of_val(&key);
            size +=  mem::size_of_val(&value);
        }
        size
    }
}