use std::collections::HashMap;
use crate::engine::collection::Collection;

#[derive(Debug, Clone)]
pub struct Database {
    name: String,
    collections: HashMap<String, Collection>, // Collections dans la base de données
}

impl Database {
    pub fn new(name: String) -> Self {
        Database {
            name,
            collections: HashMap::new(),
        }
    }

    // Méthodes pour gérer les collections
    pub fn create_collection(&mut self, name: &str) {
        let collection = Collection::new(name.to_string());
        self.collections.insert(name.to_string(), collection);
    }

    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }
}