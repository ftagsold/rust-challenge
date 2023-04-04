#![feature(future_join)]

use std::env;

use actix_web::middleware::{Logger, TrailingSlash};
use actix_web::{middleware, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use crate::services::get_sums;

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let ssl_builder = server_config();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            .service(get_sums)
    })
    .bind_openssl("127.0.0.1:8080", ssl_builder)?
    .run()
    .await
}

fn server_config() -> SslAcceptorBuilder {
    let key = env::var("KEY_FILE").unwrap_or("./localhost.key".to_string());
    let cert = env::var("CERT_FILE").unwrap_or("./localhost.crt".to_string());

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_certificate_file(cert.clone(), SslFiletype::PEM)
        .unwrap();
    builder
        .set_private_key_file(key.clone(), SslFiletype::PEM)
        .unwrap();
    builder.set_ca_file(cert.clone()).unwrap();

    builder
}
