use std::collections::HashMap;
use super::table::Table;
use super::index::Index;
use crate::models::models;

impl Schema {
    // Constructeur pour initialiser un schéma
    pub fn new(name: String) -> Self {
        Schema {
            name,
            tables: HashMap::new(),
            indexes: HashMap::new(),
        }
    }

    // Mettre à jour le nom du schéma
    pub fn update(&mut self, new_name: String) {
        self.name = new_name;
    }

    // Ajouter une table au schéma
    pub fn add_table(&mut self, table: Table) {
        self.tables.insert(table.name.clone(), table);
    }

    // Obtenir une table mutable
    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut Table> {
        self.tables.get_mut(name)
    }
    
    // Obtenir une table en lecture seule
    pub fn get_table(&self, table_name: &str) -> Option<&Table> {
        self.tables.get(table_name)
    }

    // Mettre à jour une table
    pub fn update_table(&mut self, table_name: &str, new_table: Table) -> Result<(), String> {
        if let Some(table) = self.tables.get_mut(table_name) {
            *table = new_table;
            Ok(())
        } else {
            Err(format!("Table '{}' non trouvée", table_name))
        }
    }

    // Supprimer une table
    pub fn delete_table(&mut self, table_name: &str) -> Result<(), String> {
        if self.tables.remove(table_name).is_some() {
            Ok(())
        } else {
            Err(format!("Table '{}' non trouvée", table_name))
        }
    }

    // Ajouter un index au schéma
    pub fn add_index(&mut self, index: Index) {
        self.indexes.insert(index.name.clone(), index);
    }

    // Mettre à jour un index
    pub fn update_index(&mut self, index_name: &str, new_index: Index) -> Result<(), String> {
        if self.indexes.contains_key(index_name) {
            self.indexes.insert(index_name.to_string(), new_index);
            Ok(())
        } else {
            Err(format!("Index '{}' non trouvé", index_name))
        }
    }

    // Supprimer un index
    pub fn delete_index(&mut self, index_name: &str) -> Result<(), String> {
        if self.indexes.remove(index_name).is_some() {
            Ok(())
        } else {
            Err(format!("Index '{}' non trouvé", index_name))
        }
    }

    // Trouver un index dans le schéma
    pub fn find_index(&self, index_name: &str) -> Option<&Index> {
        self.indexes.get(index_name)
    }

    // Lister toutes les tables dans le schéma
    pub fn list_tables(&self) -> Vec<&Table> {
        self.tables.values().collect()
    }
}
