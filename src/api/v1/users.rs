use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::config::{AppConfig, User}; // Assurez-vous que le chemin d'importation est correct
use std::sync::{Arc, Mutex};

// Fonction pour initialiser les routes des utilisateurs
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let engine = Arc::new(Mutex::new(Engine::new())); 
    cfg
        .app_data(web::Data::new(engine.clone())) // Partager l'instance de moteur de base de données
        .route("/users", web::post().to(add_user)) // POST /api/v1/users
        .route("/users", web::get().to(get_users)) // GET /api/v1/users
        .route("/users/{username}", web::put().to(update_user)); // PUT /api/v1/users/{username}
}

// Gestionnaire pour ajouter un utilisateur
async fn add_user(app_config: web::Data<Arc<Mutex<AppConfig>>>, user: web::Json<User>) -> impl Responder {
    let mut config = app_config.lock().unwrap();
    config.add_user(user.into_inner()); // Ajoute l'utilisateur à la configuration
    HttpResponse::Created().finish()
}

// Gestionnaire pour mettre à jour un utilisateur
async fn update_user(app_config: web::Data<Arc<Mutex<AppConfig>>>, username: web::Path<String>, user: web::Json<User>) -> impl Responder {
    let mut config = app_config.lock().unwrap();
    if config.update_user(&username, user.into_inner()) {
        HttpResponse::Ok().finish() // Utilisateur mis à jour avec succès
    } else {
        HttpResponse::NotFound().finish() // Utilisateur non trouvé
    }
}

// Gestionnaire pour récupérer les utilisateurs
async fn get_users(app_config: web::Data<Arc<Mutex<AppConfig>>>) -> impl Responder {
    let config = app_config.lock().unwrap();
    let users = config.get_users(); // Récupérer la liste des utilisateurs
    HttpResponse::Ok().json(users)
}