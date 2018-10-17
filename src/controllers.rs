use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::Json;

use models;

pub fn tokens_post() -> Result<Json<models::ServiceCatalog>> {

    let endpoint = models::Endpoint{
        public_url: "Qwdwqd".to_string(),
        admin_url: "Qwdwqd".to_string(),
        internal_url: "Qwdwqd".to_string(),
        id: "Qwdwqd".to_string(),
        region: "Qwdwqd".to_string(),
    };

    let service = models::Service {
        endpoints: vec![endpoint],
        service_type: "object-store".to_string(),
        name: "swift".to_string(),
        endpoints_links: Vec::new()
    };

    let service_catalog = models::ServiceCatalog {
        endpoints: vec![service]
    };

    Ok(Json(service_catalog))
}

pub fn account_get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn account_post() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn account_head() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Containers functions
pub fn container_head() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn container_get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn container_post() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn container_put() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn container_delete() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Object storage functions
pub fn object_head() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn object_get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn object_post() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn object_put() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn object_delete() -> HttpResponse {
    HttpResponse::Ok().finish()
}