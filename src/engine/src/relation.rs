use super::database::Database;
use crate::models::models;

impl ForeignKey {
    pub fn new(referencing_table: &str, referencing_column: &str, referenced_table: &str, referenced_column: &str) -> Self {
        Self {
            referencing_table: referencing_table.to_string(),
            referencing_column: referencing_column.to_string(),
            referenced_table: referenced_table.to_string(),
            referenced_column: referenced_column.to_string(),
        }
    }

    pub fn validate_relation(&self, db: &Database) -> Result<(), String> {
        Ok(())
    
        // Logique pour valider la relation entre les tables
    }
}
