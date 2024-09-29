use chrono::{NaiveDate, NaiveDateTime};
use std::collections::HashMap;

// Définition des structures de modèle

pub struct Column {
    pub name: String,
    data_type: DataType, // Type de données (String, Integer, etc.)
}

pub struct User {
    pub username: String,
    pub password: String,
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub users: Vec<User>, // Liste d'utilisateurs
}

pub struct Database {
    schemas: HashMap<String, Schema>,
    config: DatabaseConfig,
}

pub struct Index {
    pub name: String,
    indexed_columns: Vec<String>, // Colonnes utilisées pour l'index
    index_data: HashMap<Value, Vec<u32>>, // Valeur indexée => Liste des IDs des enregistrements
}

pub enum QueryResult {
    Records(Vec<Record>), // Un vecteur d'enregistrements retournés par une requête SELECT
    Success(String),      // Un message de succès pour les requêtes INSERT, UPDATE, DELETE
    Error(String),        // En cas d'erreur
}

// Définition de la structure Query
pub struct Query {
    // Contient des informations sur la requête SQL (ex: type, table, conditions)
}

pub enum DataType {
    Integer,
    Varchar(usize),  // Taille maximale pour les chaînes de caractères
    Float,
    Boolean,
    Date,
    Timestamp,
    Text,
    // Ajouter d'autres types selon les besoins
}

// Définition de l'énumération Value
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    String(String),
    Date(NaiveDate),
    Timestamp(NaiveDateTime),
}

// Définition de la structure Record
#[derive(Debug)]
pub struct Record {
    pub id: u32,
    values: HashMap<String, Value>, // Nom de la colonne => Valeur
}

pub struct ForeignKey {
    referencing_table: String,
    referencing_column: String,
    referenced_table: String,
    referenced_column: String,
}

pub struct Schema {
    pub name: String,
    tables: HashMap<String, Table>, // Nom de la table => Table
    indexes: HashMap<String, Index>, // Nom de l'index => Index
}

pub struct Table {
    pub name: String,
    columns: Vec<Column>,
    records: HashMap<u32, Record>, // Utiliser un HashMap pour indexer les enregistrements par ID
    ordered_ids: Vec<u32>, // Stocker les IDs dans l'ordre d'insertion
}

pub struct Transaction {
    // Gérer les changements dans une transaction
    changes: Vec<Change>, // Suivre les changements dans la transaction
}

#[derive(Debug)]
pub enum Change {
    Insert { table_name: String, record: Record },
    Update { table_name: String, record_id: u32, new_record: Record },
    Delete { table_name: String, record_id: u32 },
}
