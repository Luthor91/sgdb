use actix_web::{web, HttpResponse};
use std::sync::{Arc, Mutex};
use crate::engine::{Engine, utils}; // Assurez-vous que utils est défini dans votre module engine
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateCollectionRequest {
    name: Option<String>,
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let engine = Arc::new(Mutex::new(Engine::new())); 
    cfg
        .app_data(web::Data::new(engine.clone())) // Partager l'instance de moteur de base de données
        .route("", web::post().to(create_collection)) // POST /api/v1/collections
        .route("", web::get().to(list_collections))   // GET /api/v1/collections
        .route("/{id}", web::get().to(get_collection)) // GET /api/v1/collections/{id}
        .route("/{id}", web::delete().to(delete_collection)); // DELETE /api/v1/collections/{id}
}

pub async fn create_collection(
    data: web::Json<CreateCollectionRequest>, 
    engine: web::Data<Arc<Mutex<Engine>>>
) -> HttpResponse {
    let collection_name = match &data.name {
        Some(name) => name.clone(),
        None => utils::generate_collection_name(),
    };

    // Utiliser le Mutex pour accéder à l'Engine
    let mut engine = engine.lock().unwrap();

    match engine.create_collection(collection_name) {
        Ok(collection) => HttpResponse::Ok().json(collection),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn list_collections(
    engine: web::Data<Arc<Mutex<Engine>>>
) -> HttpResponse {
    let engine = engine.lock().unwrap();

    match engine.list_collections() {
        Ok(collections) => HttpResponse::Ok().json(collections),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn get_collection(
    collection_id: web::Path<i32>, 
    engine: web::Data<Arc<Mutex<Engine>>>
) -> HttpResponse {
    let id = collection_id.into_inner();
    let engine = engine.lock().unwrap();

    match engine.get_collection(id) {
        Some(collection) => HttpResponse::Ok().json(collection),
        None => HttpResponse::NotFound().body("Collection not found"),
    }
}

pub async fn delete_collection(
    collection_id: web::Path<i32>, 
    engine: web::Data<Arc<Mutex<Engine>>>
) -> HttpResponse {
    let id = collection_id.into_inner();
    let mut engine = engine.lock().unwrap();

    match engine.delete_collection(id) {
        Ok(_) => HttpResponse::Ok().body("Collection deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}