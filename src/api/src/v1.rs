// src/api/v1.rs
use models::update_models::*;
use models::models::*;
use log::Record;
use serde_json::json;
use actix_web::{web, post, get, put, delete, HttpResponse, Responder};

// GET /api/v1/databases
#[get("/api/v1/databases")]
async fn list_databases() -> impl Responder {
    let databases = Database::list_all_databases(); // Modifié pour appeler la méthode correcte
    HttpResponse::Ok().json(databases) // Renvoie les bases de données en JSON
}

// GET /api/v1/{database}
#[get("/api/v1/{database}")]
async fn get_database(path: web::Path<String>) -> impl Responder {
    let database = path.into_inner();
    match Database::get_database_info(&database) { // Appelle la méthode pour obtenir les détails de la DB
        Some(info) => HttpResponse::Ok().json(info), // Retourne les informations de la DB en JSON
        None => HttpResponse::NotFound().body(format!("Database '{}' not found", database))
    }
}

// POST /api/v1/{database}
#[post("/api/v1/{database}")]
async fn create_database(path: web::Path<String>) -> impl Responder {
    let database = path.into_inner();
    match Database::create_new_database(&database) {
        Ok(_) => HttpResponse::Ok().body(format!("Database '{}' created successfully!", database)),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

// PUT /api/v1/{database}
#[put("/api/v1/{database}")]
async fn update_database(path: web::Path<String>, body: web::Json<DatabaseUpdate>) -> impl Responder {
    let database = path.into_inner();
    match Database::update_database(&database, body.into_inner()) {
        Ok(_) => HttpResponse::Ok().body(format!("Database '{}' updated successfully!", database)),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// DELETE /api/v1/{database}
#[delete("/api/v1/{database}")]
async fn delete_database(path: web::Path<String>) -> impl Responder {
    let database = path.into_inner();
    match Database::delete_existing_database(&database) {
        Ok(_) => HttpResponse::Ok().body(format!("Database '{}' deleted successfully!", database)),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// GET /api/v1/{database}/schemas
#[get("/api/v1/{database}/schemas")]
async fn list_schemas(path: web::Path<String>) -> impl Responder {
    let database = path.into_inner();
    match Database::list_schemas_in_database(&database) {
        Ok(schemas) => HttpResponse::Ok().json(schemas),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// GET /api/v1/{database}/{schema}
#[get("/api/v1/{database}/{schema}")]
async fn get_schema(path: web::Path<(String, String)>) -> impl Responder {
    let (database, schema) = path.into_inner();
    match Database::get_schema(&database, &schema) { // Appel modifié pour obtenir le schéma
        Some(info) => HttpResponse::Ok().json(info),
        None => HttpResponse::NotFound().body(format!("Schema '{}' not found in database '{}'", schema, database)),
    }
}

// POST /api/v1/{database}/{schema}
#[post("/api/v1/{database}/{schema}")]
async fn create_schema(path: web::Path<(String, String)>) -> impl Responder {
    let (database, schema) = path.into_inner();
    match Database::create_new_schema(&database, &schema) {
        Ok(_) => HttpResponse::Ok().body(format!("Schema '{}' created in database '{}'.", schema, database)),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

// PUT /api/v1/{database}/{schema}
#[put("/api/v1/{database}/{schema}")]
async fn update_schema(path: web::Path<(String, String)>, body: web::Json<SchemaUpdate>) -> impl Responder {
    let (database, schema) = path.into_inner();
    match Database::update_schema(&database, &schema, body.into_inner()) {
        Ok(_) => HttpResponse::Ok().body(format!("Schema '{}' updated in database '{}'.", schema, database)),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// DELETE /api/v1/{database}/{schema}
#[delete("/api/v1/{database}/{schema}")]
async fn delete_schema(path: web::Path<(String, String)>) -> impl Responder {
    let (database, schema) = path.into_inner();
    match Database::delete_existing_schema(&database, &schema) {
        Ok(_) => HttpResponse::Ok().body(format!("Schema '{}' deleted from database '{}'.", schema, database)),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// GET /api/v1/{database}/{schema}/tables
#[get("/api/v1/{database}/{schema}/tables")]
async fn list_tables(path: web::Path<(String, String)>) -> impl Responder {
    let (database, schema) = path.into_inner();
    match Database::list_tables_in_schema(&database, &schema) {
        Ok(tables) => HttpResponse::Ok().json(tables),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// GET /api/v1/{database}/{schema}/{table}
#[get("/api/v1/{database}/{schema}/{table}")]
async fn get_table(path: web::Path<(String, String, String)>) -> impl Responder {
    let (database, schema, table) = path.into_inner();
    match Database::get_table_info(&database, &schema, &table) {
        Some(info) => HttpResponse::Ok().json(info),
        None => HttpResponse::NotFound().body(format!("Table '{}' not found in schema '{}' in database '{}'", table, schema, database)),
    }
}

// POST /api/v1/{database}/{schema}/{table}
#[post("/api/v1/{database}/{schema}/{table}")]
async fn create_table(path: web::Path<(String, String, String)>) -> impl Responder {
    let (database, schema, table) = path.into_inner();
    match Database::create_new_table(&database, &schema, &table) {
        Ok(_) => HttpResponse::Ok().body(format!("Table '{}' created in schema '{}' in database '{}'.", table, schema, database)),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

// PUT /api/v1/{database}/{schema}/{table}
#[put("/api/v1/{database}/{schema}/{table}")]
async fn update_table(path: web::Path<(String, String, String)>, body: web::Json<TableUpdate>) -> impl Responder {
    let (database, schema, table) = path.into_inner();
    match Database::update_table(&database, &schema, &table, body.into_inner()) {
        Ok(_) => HttpResponse::Ok().body(format!("Table '{}' updated in schema '{}' in database '{}'.", table, schema, database)),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// DELETE /api/v1/{database}/{schema}/{table}
#[delete("/api/v1/{database}/{schema}/{table}")]
async fn delete_table(path: web::Path<(String, String, String)>) -> impl Responder {
    let (database, schema, table) = path.into_inner();
    match Database::delete_existing_table(&database, &schema, &table) {
        Ok(_) => HttpResponse::Ok().body(format!("Table '{}' deleted from schema '{}' in database '{}'.", table, schema, database)),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// GET /api/v1/{database}/{schema}/{table}/columns
#[get("/api/v1/{database}/{schema}/{table}/columns")]
async fn list_columns(path: web::Path<(String, String, String)>) -> impl Responder {
    let (database, schema, table) = path.into_inner();
    match Database::list_columns_in_table(&database, &schema, &table) {
        Ok(columns) => HttpResponse::Ok().json(columns),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// GET /api/v1/{database}/{schema}/{table}/{column}
#[get("/api/v1/{database}/{schema}/{table}/{column}")]
async fn get_column(path: web::Path<(String, String, String, String)>) -> impl Responder {
    let (database, schema, table, column) = path.into_inner();
    match Database::get_column_info(&database, &schema, &table, &column) {
        Ok(column_info) => HttpResponse::Ok().json(column_info),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// POST /api/v1/{database}/{schema}/{table}/{column}
#[post("/api/v1/{database}/{schema}/{table}/{column}")]
async fn add_column(path: web::Path<(String, String, String, String)>) -> impl Responder {
    let (database, schema, table, column) = path.into_inner();   
    match Database::add_column(&database, &schema, &table, &column) {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": format!("Column '{}' added to table '{}' in schema '{}' in database '{}'.", column, table, schema, database)
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// PUT /api/v1/{database}/{schema}/{table}/{column}
#[put("/api/v1/{database}/{schema}/{table}/{column}")]
async fn update_column(path: web::Path<(String, String, String, String)>, body: web::Json<ColumnUpdate>) -> impl Responder {
    let (database, schema, table, column) = path.into_inner();
    match Database::update_column(&database, &schema, &table, &column, body.into_inner()) {
        Ok(_) => HttpResponse::Ok().body(format!("Column '{}' updated in table '{}' in schema '{}' in database '{}'.", column, table, schema, database)),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// DELETE /api/v1/{database}/{schema}/{table}/{column}
#[delete("/api/v1/{database}/{schema}/{table}/{column}")]
async fn delete_column(path: web::Path<(String, String, String, String)>) -> impl Responder {
    let (database, schema, table, column) = path.into_inner();   
    match Database::delete_column(&database, &schema, &table, &column) {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": format!("Column '{}' deleted from table '{}' in schema '{}' in database '{}'.", column, table, schema, database)
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// GET /api/v1/{database}/{schema}/{table}/{column}/records
#[get("/api/v1/{database}/{schema}/{table}/{column}/records")]
async fn list_records(path: web::Path<(String, String, String)>) -> impl Responder {
    let (database, schema, table) = path.into_inner();
    match Database::list_records(&database, &schema, &table) {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// GET /api/v1/{database}/{schema}/{table}/{column}/records/{id}
#[get("/api/v1/{database}/{schema}/{table}/{column}/records/{id}")]
async fn get_record(path: web::Path<(String, String, String, u32)>) -> impl Responder {
    let (database, schema, table, id) = path.into_inner();
    match Database::get_record(&database, &schema, &table, id) {
        Ok(record) => HttpResponse::Ok().json(record),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// POST /api/v1/{database}/{schema}/{table}/{column}/records
#[post("/api/v1/{database}/{schema}/{table}/{column}/records")]
async fn insert_record(path: web::Path<(String, String, String, String)>, body: web::Json<Record>) -> impl Responder {
    let (database, schema, table, column) = path.into_inner();
    match Database::insert_record(&database, &schema, &table, &column, body.into_inner()) {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": format!("Record inserted into column '{}' in table '{}' in schema '{}' in database '{}'.", column, table, schema, database)
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// PUT /api/v1/{database}/{schema}/{table}/{column}/records/{id}
#[put("/api/v1/{database}/{schema}/{table}/{column}/records/{id}")]
async fn update_record(path: web::Path<(String, String, String, String, u32)>, body: web::Json<RecordUpdate>) -> impl Responder {
    let (database, schema, table, column, id) = path.into_inner();
    match Database::update_record(&database, &schema, &table, &column, id, body.into_inner()) {
        Ok(_) => HttpResponse::Ok().json(json!( {
            "message": format!("Record with id '{}' updated in column '{}' in table '{}' in schema '{}' in database '{}'.", id, column, table, schema, database)
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// DELETE /api/v1/{database}/{schema}/{table}/{column}/records/{id}
#[delete("/api/v1/{database}/{schema}/{table}/{column}/records/{id}")]
async fn delete_record(path: web::Path<(String, String, String, String, u32)>) -> impl Responder {
    let (database, schema, table, column, id) = path.into_inner();
    match Database::delete_record(&database, &schema, &table, &column, id) {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": format!("Record deleted from column '{}' in table '{}' in schema '{}' in database '{}'.", column, table, schema, database)
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
