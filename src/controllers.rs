use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::Json;

use models;

pub fn tokens_post(req: &HttpRequest) -> Result<Json<models::Access>> {

    println!("{:?}", req.uri());

    let endpoint = models::Endpoint{
        public_url: "TODO".to_string(),
        admin_url: "TODO".to_string(),
        internal_url: "TODO".to_string(),
        id: "TODO".to_string(),
        region: "TODO".to_string(),
    };

    let service = models::Service {
        endpoints: vec![endpoint],
        service_type: "object-store".to_string(),
        name: "swift".to_string(),
        endpoints_links: Vec::new()
    };

    let access = models::Access {
        service_catalog: models::ServiceCatalog {
            endpoints: vec![service]
        },
        token: models::Token {
            id: "TODO".to_string(),
            issued_at: "TODO".to_string(),
            expires: "TODO".to_string(),
            tenant: models::TokenTenant {
                id: "TODO".to_string(),
                description: "TODO".to_string(),
                enabled: true,
                name: "TODO".to_string()
            }
        },
        user: models::User {
            name: "TODO".to_string(),
            username: "TODO".to_string(),
            roles: vec![],
            roles_links: vec![]
        },
        metadata: models::Metadata {
            is_admin: true,
            roles: vec![]
        }
    };

    Ok(Json(access))
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