use std::collections::HashMap;
use crate::engine::database;

// Structure principale du moteur de base de données
#[derive(Clone)]
pub struct Engine {
    databases: HashMap<String, database::Database>, // Liste des bases de données
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            databases: HashMap::new(),
        }
    }

    // Méthodes pour gérer les bases de données
    pub fn create_database(&mut self, name: &str) {
        let db = database::Database::new(name.to_string());
        self.databases.insert(name.to_string(), db);
    }

    pub fn get_database(&self, name: &str) -> Option<&database::Database> {
        self.databases.get(name)
    }
}