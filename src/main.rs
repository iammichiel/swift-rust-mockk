extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use actix_web::server;
use actix_web::App;
use actix_web::http::Method;

pub mod controllers;
pub mod models;

fn main() {
    server::new(|| {
        App::new()

            .resource("/v2/tokens", |r| {
                r.method(Method::POST).f(|req| controllers::tokens_post(req))
            })

            .resource("/v1/{account}", |r| {
                r.method(Method::GET).f(|_| controllers::account_get());
                r.method(Method::HEAD).f(|_| controllers::account_head());
                r.method(Method::POST).f(|_| controllers::account_post())
            })

            .resource("/v1/{account}/{container}", |r| {
                r.method(Method::HEAD).f(|_| controllers::container_head());
                r.method(Method::GET).f(|_| controllers::container_get());
                r.method(Method::POST).f(|_| controllers::container_post());
                r.method(Method::PUT).f(|_| controllers::container_put());
                r.method(Method::DELETE).f(|_| controllers::container_delete())
            })

            .resource("/v1/{account}/{container}/{object}", |r| {
                r.method(Method::HEAD).f(|_| controllers::object_head());
                r.method(Method::GET).f(|_| controllers::object_get());
                r.method(Method::POST).f(|_| controllers::object_post());
                r.method(Method::PUT).f(|_| controllers::object_put());
                r.method(Method::DELETE).f(|_| controllers::object_delete())
            })
    })
    .bind("127.0.0.1:8888")
    .expect("Can not bind to port 8888")
    .run();
}
