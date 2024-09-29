use crate::models::models;
use super::record::Value;
use std::collections::HashMap;

impl Index {
    pub fn new(index_name: &str, indexed_columns: Vec<String>) -> Self {
        Self {
            name: index_name.to_string(),
            indexed_columns,
            index_data: HashMap::new(),
        }
    }

    // Ajouter un enregistrement à l'index
    pub fn add_to_index(&mut self, value: Value, record_id: u32) {
        self.index_data
            .entry(value)
            .or_insert_with(Vec::new) // Crée une nouvelle entrée si elle n'existe pas
            .push(record_id);          // Ajoute l'ID de l'enregistrement
    }

    // Trouver les enregistrements associés à une valeur dans l'index
    pub fn find_in_index(&self, value: &Value) -> Option<&Vec<u32>> {
        self.index_data.get(value)
    }
}
