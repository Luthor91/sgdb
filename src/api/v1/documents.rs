use actix_web::{web, HttpResponse};
use std::sync::{Arc, Mutex};
use crate::engine::{Engine, Document}; // Assurez-vous que Document est défini dans votre module Engine
use serde::Deserialize;

#[derive(Deserialize)]
struct DocumentRequest {
    data: serde_json::Value, // Utilisation de serde_json::Value pour les données
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let engine = Arc::new(Mutex::new(Engine::new())); 
    cfg
        .app_data(web::Data::new(engine.clone())) // Partager l'instance de moteur de base de données
        .route("/{collection_id}/documents", web::post().to(add_document)) // POST /api/v1/collections/{id}/documents
        .route("/{collection_id}/documents/{doc_id}", web::get().to(get_document)) // GET /api/v1/collections/{id}/documents/{doc_id}
        .route("/{collection_id}/documents/{doc_id}", web::put().to(update_document)) // PUT /api/v1/collections/{id}/documents/{doc_id}
        .route("/{collection_id}/documents/{doc_id}", web::delete().to(delete_document)); // DELETE /api/v1/collections/{id}/documents/{doc_id}
}

pub async fn add_document(
    collection_id: web::Path<i32>, 
    document: web::Json<DocumentRequest>, 
    engine: web::Data<Arc<Mutex<Engine>>>
) -> HttpResponse {
    let id = collection_id.into_inner();
    let mut engine = engine.lock().unwrap();

    match engine.add_document(id, document.data.clone()) { // Cloner les données
        Ok(doc_id) => HttpResponse::Ok().json(doc_id),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn get_document(
    path: web::Path<(i32, i32)>, 
    engine: web::Data<Arc<Mutex<Engine>>>
) -> HttpResponse {
    let (collection_id, document_id) = path.into_inner();
    let engine = engine.lock().unwrap();

    match engine.get_document(collection_id, document_id) {
        Some(document) => HttpResponse::Ok().json(document), // Document doit être sérialisable en JSON
        None => HttpResponse::NotFound().body("Document not found"),
    }
}

pub async fn update_document(
    path: web::Path<(i32, i32)>, 
    document: web::Json<DocumentRequest>, 
    engine: web::Data<Arc<Mutex<Engine>>>
) -> HttpResponse {
    let (collection_id, document_id) = path.into_inner();
    let mut engine = engine.lock().unwrap();

    match engine.update_document(collection_id, document_id, document.data.clone()) { // Cloner les données
        Ok(_) => HttpResponse::Ok().body("Document updated"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn delete_document(
    path: web::Path<(i32, i32)>, // Utilisation de (collection_id, doc_id)
    engine: web::Data<Arc<Mutex<Engine>>>,
) -> HttpResponse {
    let (collection_id, doc_id) = path.into_inner();
    let mut engine = engine.lock().unwrap();

    match engine.delete_document(collection_id, doc_id) { // Assurez-vous d'avoir cette méthode dans Engine
        Ok(_) => HttpResponse::Ok().body("Document deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}