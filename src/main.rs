use actix_web::{App, HttpServer};
use api::v1::*; // Importation des routes de l'API définies dans le package api

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(list_databases)
            .service(get_database)
            .service(create_database)
            .service(update_database)
            .service(delete_database)
            .service(list_schemas)
            .service(get_schema)
            .service(create_schema)
            .service(update_schema)
            .service(delete_schema)
            .service(list_tables)
            .service(get_table)
            .service(create_table)
            .service(update_table)
            .service(update_table)
            .service(delete_table)
            .service(list_columns)
            .service(get_column)
            .service(add_column)
            .service(update_column)
            .service(delete_column)
            .service(list_records)
            .service(get_record)
            .service(insert_record)
            .service(update_record)
            .service(delete_record)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
