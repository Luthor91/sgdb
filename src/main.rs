mod config;
mod api; // Assurez-vous d'importer le module api
mod engine; // Importer le module engine

use actix_web::{App, HttpServer, web::*};
use actix_web::web;
use config::AppConfig;
use std::sync::{Arc, Mutex};

// Assurez-vous que vous avez un chemin d'importation correct pour vos modules et sous-modules

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiser la configuration de l'application avec des paramètres définis
    let mut app_config = AppConfig::new();

    // Exemple de configuration des utilisateurs (à adapter selon votre logique d'application)
    let user1 = config::User {
        username: "admin".to_string(),
        password: Some("password".to_string()), // Mot de passe haché recommandé pour la sécurité
        accessible_databases: vec!["ma_base_de_donnees".to_string()],
    };
    app_config.engine.lock().unwrap().add_user(user1);

    // Créer une base de données et une collection pour l'exemple
    {
        let mut engine = app_config.engine.lock().unwrap();
        engine.create_database("ma_base_de_donnees");

        if let Some(db) = engine.get_database("ma_base_de_donnees") {
            db.create_collection("ma_collection");

            // Ajouter un document à la collection
            let document = engine::document::Document::new(1, serde_json::json!({"name": "Alice", "age": 30}));
            db.get_collection("ma_collection").unwrap().add_document(document);
        }
    }

    // Lancer le serveur HTTP
    let server_ip = app_config.config.ip.clone(); // Récupérer l'adresse IP de la configuration
    let server_port = app_config.config.port; // Récupérer le port de la configuration

    HttpServer::new(move || {
        let engine_clone = app_config.engine.clone(); // Cloner l'Arc pour chaque instance de serveur
        App::new()
            .app_data(Data::new(engine_clone)) // Partager l'instance de moteur de base de données
            .service(web::scope("/api/v1")
                .configure(api::v1::collections::init_routes) // Initialiser les routes pour les collections
                .configure(api::v1::documents::init_routes)   // Initialiser les routes pour les documents
            )
    })
    .bind(format!("{}:{}", server_ip, server_port))? // Utiliser l'adresse IP et le port de la configuration
    .run()
    .await
}   