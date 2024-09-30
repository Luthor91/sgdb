use std::collections::HashMap;
use crate::engine::database;
use crate::config::{DatabaseConfig, User}; // Assurez-vous que le chemin d'importation est correct

// Structure principale du moteur de base de données
#[derive(Clone)]
pub struct Engine {
    databases: HashMap<String, database::Database>, // Liste des bases de données
    config: DatabaseConfig, // Configuration du SGBD
}

impl Engine {
    pub fn new(config: DatabaseConfig) -> Self {
        Engine {
            databases: HashMap::new(),
            config,
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

    // Méthodes pour gérer les utilisateurs
    pub fn add_user(&mut self, user: User) {
        self.config.add_user(user);
    }

    pub fn get_user(&self, username: &str) -> Option<&User> {
        self.config.users.get(username)
    }

    // Méthode pour vérifier l'authentification de l'utilisateur
    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        if let Some(user) = self.get_user(username) {
            match &user.password {
                Some(pwd) => pwd == password, // Comparer le mot de passe haché si nécessaire
                None => true, // Authentifier sans mot de passe si optionnel
            }
        } else {
            false // Utilisateur non trouvé
        }
    }
}