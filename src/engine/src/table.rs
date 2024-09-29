use std::collections::HashMap;
use super::{column::Column, record::Record};
use crate::models::models;

impl Table {
    pub fn new(name: &str, columns: Vec<Column>) -> Self {
        Self {
            name: name.to_string(),
            columns,
            records: HashMap::new(),
            ordered_ids: Vec::new(),
        }
    }

    // Insérer un enregistrement dans une table
    pub fn insert(&mut self, record: Record) -> Result<(), String> {
        let record_id = record.id;
        if self.records.contains_key(&record_id) {
            return Err(format!("Enregistrement avec ID '{}' existe déjà", record_id));
        }
        self.records.insert(record_id, record);
        self.ordered_ids.push(record_id);
        Ok(())
    }

    // Rechercher un enregistrement dans la table
    pub fn find(&self, record_id: u32) -> Option<&Record> {
        self.records.get(&record_id)
    }

    // Récupérer un enregistrement par son index ordinal
    pub fn get_by_index(&self, index: usize) -> Option<&Record> {
        if index < self.ordered_ids.len() {
            let record_id = self.ordered_ids[index];
            self.records.get(&record_id)
        } else {
            None
        }
    }

    // Mettre à jour un enregistrement dans la table
    pub fn update(&mut self, record_id: u32, new_data: Record) -> Result<(), String> {
        if self.records.contains_key(&record_id) {
            self.records.insert(record_id, new_data);
            Ok(())
        } else {
            Err(format!("Enregistrement avec ID '{}' non trouvé", record_id))
        }
    }

    // Supprimer un enregistrement dans la table
    pub fn delete(&mut self, record_id: u32) -> Result<(), String> {
        if self.records.remove(&record_id).is_none() {
            Err(format!("Enregistrement avec ID '{}' non trouvé", record_id))
        } else {
            self.ordered_ids.retain(|&id| id != record_id);
            Ok(())
        }
    }

    // Lister tous les enregistrements
    pub fn list_records(&self) -> Vec<&Record> {
        self.ordered_ids.iter().filter_map(|&id| self.records.get(&id)).collect()
    }
}
