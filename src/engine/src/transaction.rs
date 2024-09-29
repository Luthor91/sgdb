use super::record::Record;
use super::schema::Schema;
use crate::models::models;

impl Transaction {
    pub fn new() -> Self {
        // Démarrer une nouvelle transaction
        Self {
            changes: Vec::new(),
        }
    }

    pub fn commit(self, schema: &mut Schema) -> Result<(), String> {
        // Appliquer les changements à la base de données
        for change in self.changes {
            match change {
                Change::Insert { table_name, record } => {
                    // Logique pour insérer un enregistrement dans la table correspondante
                    if let Some(table) = schema.get_table_mut(&table_name) { // Utiliser get_table_mut ici
                        table.insert(record)?; // Assurez-vous que table.insert() accepte &mut self
                    } else {
                        return Err(format!("Table '{}' non trouvée.", table_name));
                    }
                }
                Change::Update { table_name, record_id, new_record } => {
                    // Logique pour mettre à jour un enregistrement dans la table correspondante
                    if let Some(table) = schema.get_table_mut(&table_name) { // Utiliser get_table_mut ici
                        table.update(record_id, new_record)?; // Assurez-vous que table.update() accepte &mut self
                    } else {
                        return Err(format!("Table '{}' non trouvée.", table_name));
                    }
                }
                Change::Delete { table_name, record_id } => {
                    // Logique pour supprimer un enregistrement dans la table correspondante
                    if let Some(table) = schema.get_table_mut(&table_name) { // Utiliser get_table_mut ici
                        table.delete(record_id)?; // Assurez-vous que table.delete() accepte &mut self
                    } else {
                        return Err(format!("Table '{}' non trouvée.", table_name));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn rollback(self) -> Result<(), String> {
        // Annuler les changements dans la base de données (ici, pas d'implémentation spécifique)
        // L'idée étant de ne pas appliquer les changements
        Ok(())
    }

    // Ajouter des méthodes pour gérer les changements
    pub fn add_insert(&mut self, table_name: String, record: Record) {
        self.changes.push(Change::Insert { table_name, record });
    }

    pub fn add_update(&mut self, table_name: String, record_id: u32, new_record: Record) {
        self.changes.push(Change::Update {
            table_name,
            record_id,
            new_record,
        });
    }

    pub fn add_delete(&mut self, table_name: String, record_id: u32) {
        self.changes.push(Change::Delete { table_name, record_id });
    }
}