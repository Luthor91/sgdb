mod config;
mod api; // Assure-toi d'importer le module api
mod engine; // Importer le module engine

use actix_web::{App, HttpServer, web::*};
use actix_web::web;
use config::AppConfig::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiser la configuration de l'application
    let app_config = AppConfig::new();
    let engine = app_config.engine.clone(); // Cloner l'Arc pour une utilisation ultérieure

    // Créer une base de données et une collection pour l'exemple
    engine.lock().unwrap().create_database("ma_base_de_donnees");

    if let Some(db) = engine.lock().unwrap().get_database("ma_base_de_donnees") {
        db.create_collection("ma_collection");

        // Ajouter un document à la collection
        let document = engine::document::Document::new(1, serde_json::json!({"name": "Alice", "age": 30}));
        db.get_collection("ma_collection").unwrap().add_document(document);
    }

    // Lancer le serveur HTTP
    HttpServer::new(move || {
        let engine_clone = engine.clone(); // Cloner l'Arc pour chaque instance de serveur
        App::new()
            .app_data(Data::new(engine_clone)) // Partager l'instance de moteur de base de données
            .service(web::scope("/api/v1")
                .configure(api::v1::collections::init_routes) // Initialiser les routes pour les collections
                .configure(api::v1::documents::init_routes)   // Initialiser les routes pour les documents
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
