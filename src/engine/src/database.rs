use std::collections::HashMap;
use crate::models::models;
use crate::engine::schema::Schema;
use super::config::DatabaseConfig;

impl Database {
    // Constructeur pour initialiser Database
    pub fn new(config: DatabaseConfig) -> Self {
        Database {
            schemas: HashMap::new(),
            config,
        }
    }

    // Ajouter un schéma
    pub fn add_schema(&mut self, schema: Schema) {
        self.schemas.insert(schema.name.clone(), schema);
    }

    // Supprimer un schéma
    pub fn delete_schema(&mut self, schema_name: &str) -> Result<(), String> {
        if self.schemas.remove(schema_name).is_some() {
            Ok(())
        } else {
            Err(format!("Schéma '{}' non trouvé", schema_name))
        }
    }

    // Obtenir un schéma
    pub fn get_schema(&self, schema_name: &str) -> Option<&Schema> {
        self.schemas.get(schema_name)
    }

    // Liste de tous les schémas
    pub fn list_schemas(&self) -> Vec<&Schema> {
        self.schemas.values().collect()
    }
}
