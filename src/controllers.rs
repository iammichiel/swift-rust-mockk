use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::Json;

use models;

pub fn tokens_post(req: &HttpRequest) -> Result<Json<models::TokenRequestResponse>> {

    let full_hostname = format!("{}://{}", req.connection_info().scheme(), req.connection_info().host());
    let endpoint_url = format!("{}/", full_hostname);
    println!("{}", full_hostname);

    let service = models::Service {
        endpoints: vec![models::Endpoint{
            public_url: endpoint_url.clone(),
            admin_url: endpoint_url.clone(),
            internal_url: endpoint_url.clone(),
            id: "SBG1".to_string(),
            region: "SBG1".to_string(),
        }],
        service_type: "object-store".to_string(),
        name: "swift".to_string(),
        endpoints_links: Vec::new()
    };

    let access = models::TokenRequestResponse {
        access: models::Access {
            service_catalog: vec![service],
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