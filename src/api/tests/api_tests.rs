#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_list_databases() {
        let app = test::init_service(
            App::new().service(list_databases)
        ).await;
        
        let req = test::TestRequest::get()
            .uri("/api/v1/databases")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
