use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::engine::{Engine, database}; // Assurez-vous que le chemin d'importation est correct
use serde::{Serialize, Deserialize};

// Structure pour représenter un utilisateur
#[derive(Clone)]
pub struct User {
    pub username: String,
    pub password: Option<String>,
    pub accessible_databases: Vec<String>,
}

// Structure pour représenter la configuration du SGBD
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub ip_address: String,
    pub port: u16,
    pub users: HashMap<String, User>, // Liste des utilisateurs
}

// Implémentation de la configuration
impl DatabaseConfig {
    pub fn new(ip_address: &str, port: u16) -> Self {
        DatabaseConfig {
            ip_address: ip_address.to_string(),
            port,
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.username.clone(), user);
    }
}

// Structure principale de la configuration de l'application
pub struct AppConfig {
    pub engine: Arc<Mutex<Engine>>,
    pub users: HashMap<String, User>, // Dictionnaire pour stocker les utilisateurs
}

impl AppConfig {
    pub fn new() -> Self {
        let engine = Arc::new(Mutex::new(Engine::new())); // Assurez-vous que votre Engine a un constructeur approprié
        let users = HashMap::new();
        AppConfig { engine, users }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.username.clone(), user);
    }

    pub fn update_user(&mut self, username: &str, user: User) -> bool {
        if self.users.contains_key(username) {
            self.users.insert(username.to_string(), user);
            true
        } else {
            false
        }
    }

    pub fn get_users(&self) -> Vec<User> {
        self.users.values().cloned().collect()
    }

}