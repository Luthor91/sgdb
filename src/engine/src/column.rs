use crate::engine::record::Value;
use crate::models::models;

impl Column {
    pub fn new(name: &str, data_type: DataType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
        }
    }

    // Valider les types de données lors de l'insertion
    pub fn validate(&self, value: &Value) -> Result<(), String> {
        match (&self.data_type, value) {
            (DataType::Integer, Value::Int(_)) => Ok(()),
            (DataType::Varchar(max_size), Value::String(s)) if s.len() <= *max_size => Ok(()),
            (DataType::Boolean, Value::Bool(_)) => Ok(()),
            (DataType::Date, Value::Date(_)) => Ok(()),
            (DataType::Timestamp, Value::Timestamp(_)) => Ok(()),
            (DataType::Text, Value::String(_)) => Ok(()), // Pour des chaînes de texte longues
            // Ajouter d'autres validations selon les types de données
            _ => Err(format!("Type de valeur invalide pour la colonne '{}'", self.name)),
        }
    }

    // Obtenir le nom de la colonne
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
