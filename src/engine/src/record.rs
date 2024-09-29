use crate::models::models;
use std::collections::HashMap;
use chrono::{NaiveDate, NaiveDateTime};


impl Record {
    pub fn new(id: u32, values: HashMap<String, Value>) -> Self {
        Self { id, values }
    }

    // Obtenir la valeur d'un champ
    pub fn get(&self, column_name: &str) -> Option<&Value> {
        self.values.get(column_name)
    }

    // Mettre à jour la valeur d'un champ
    pub fn update(&mut self, column_name: String, value: Value) -> Result<(), String> {
        if self.values.contains_key(&column_name) {
            self.values.insert(column_name, value);
            Ok(())
        } else {
            Err(format!("Colonne '{}' non trouvée", column_name))
        }
    }
}
