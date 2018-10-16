extern crate actix_web;

use actix_web::{server, App, HttpResponse};
use actix_web::http::Method;

pub mod controllers;

fn main() {
    server::new(|| {
        App::new()
            .resource("/v1/{account}", |r| {
                r.method(Method::GET).f(|_| controllers::account_get());
                r.method(Method::HEAD).f(|_| controllers::account_head());
                r.method(Method::POST).f(|_| controllers::account_post())
            })

            .resource("/v1/{account}/{container}", |r| {
                r.method(Method::GET).f(|_| HttpResponse::Ok());
                r.method(Method::POST).f(|_| HttpResponse::Ok());
                r.method(Method::PUT).f(|_| HttpResponse::Ok());
                r.method(Method::DELETE).f(|_| HttpResponse::Ok());
                r.method(Method::HEAD).f(|_| HttpResponse::Ok())
            })

            .resource("/v1/{account}/{container}/{object}", |r| {
                r.method(Method::GET).f(|_| HttpResponse::Ok());
                r.method(Method::POST).f(|_| HttpResponse::Ok());
                r.method(Method::PUT).f(|_| HttpResponse::Ok());
                r.method(Method::DELETE).f(|_| HttpResponse::Ok());
                r.method(Method::HEAD).f(|_| HttpResponse::Ok())
            })
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
