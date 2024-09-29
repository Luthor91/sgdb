use crate::models::models;

impl DatabaseConfig {

    // Méthode pour créer une nouvelle instance de DatabaseConfig
    pub fn new(host: String, port: u16, users: Vec<User>) -> Self {
        DatabaseConfig {
            host,
            port,
            users,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.host.is_empty() {
            return Err("Host cannot be empty.".to_string());
        }
        if self.port == 0 {
            return Err("Port must be a valid number between 1 and 65535.".to_string());
        }
        // Vérifier si au moins un utilisateur a un nom d'utilisateur valide
        if self.users.is_empty() {
            return Err("User list cannot be empty.".to_string());
        }
        for user in &self.users {
            if user.username.is_empty() {
                return Err("Username cannot be empty.".to_string());
            }
            // Optionnel : vérifier si le mot de passe est suffisamment fort
            if user.password.len() < 8 { // Exemple de vérification de la force du mot de passe
                return Err("Password must be at least 8 characters long.".to_string());
            }
        }
        Ok(())
    }
}