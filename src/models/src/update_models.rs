use std::collections::HashMap;

use super::models::*;

pub struct ColumnUpdate {
    pub name: Option<String>,       // Nouveau nom de la colonne (optionnel)
    pub data_type: Option<DataType>, // Nouveau type de données (optionnel)
}

pub struct UserUpdate {
    pub username: Option<String>,    // Nouveau nom d'utilisateur (optionnel)
    pub password: Option<String>,     // Nouveau mot de passe (optionnel)
}

pub struct DatabaseConfigUpdate {
    pub host: Option<String>,         // Nouvelle adresse hôte (optionnel)
    pub port: Option<u16>,            // Nouveau port (optionnel)
    pub users: Option<Vec<User>>,     // Liste mise à jour d'utilisateurs (optionnel)
}

pub struct DatabaseUpdate {
    pub config: Option<DatabaseConfigUpdate>, // Nouveau config de la base de données (optionnel)
}

pub struct IndexUpdate {
    pub name: Option<String>,             // Nouveau nom de l'index (optionnel)
    pub indexed_columns: Option<Vec<String>>, // Colonnes indexées mises à jour (optionnel)
}

pub struct SchemaUpdate {
    pub name: Option<String>,             // Nouveau nom du schéma (optionnel)
}

pub struct TableUpdate {
    pub name: Option<String>,             // Nouveau nom de la table (optionnel)
    pub columns: Option<Vec<ColumnUpdate>>, // Colonnes mises à jour (optionnel)
}

pub struct RecordUpdate {
    pub id: u32,                         // ID de l'enregistrement à mettre à jour
    pub values: HashMap<String, Value>,  // Valeurs mises à jour (nom de la colonne => nouvelle valeur)
}

pub struct ForeignKeyUpdate {
    pub referencing_table: Option<String>,   // Nouvelle table référente (optionnel)
    pub referencing_column: Option<String>,   // Nouvelle colonne référente (optionnel)
    pub referenced_table: Option<String>,     // Nouvelle table référencée (optionnel)
    pub referenced_column: Option<String>,     // Nouvelle colonne référencée (optionnel)
}
