use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: i32,
    pub data: Value, // Peut stocker n'importe quel type de données JSON
}

impl Document {
    pub fn new(id: i32, data: Value) -> Self {
        Document { id, data }
    }
}