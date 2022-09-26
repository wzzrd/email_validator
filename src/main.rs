mod oas;
mod oauth2;
mod schemas;

extern crate serde_derive;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, Error, FromRequest, HttpRequest, HttpServer};
use check_if_email_exists::syntax::{check_syntax, SyntaxDetails};
use gethostname::gethostname;
use log;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
    Apiv2Schema, Apiv2Security, OpenApiExt,
};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

const VERSION: &str = env!("CARGO_PKG_VERSION", "Cargo.toml is missing a version number.");
const GATEWAY: &str = env!("GATEWAY", "Please set the GATEWAY environment variable.");

#[api_v2_operation(
    summary = "Simple validation of a single email address",
    description = "Returns a JSON object containing information on validity of email address, and the components of that address.",
    operation_id = "Validate email address",
    consumes = "application/json",
    produces = "application/json"
)]
async fn validate_address(
    a: Json<schemas::Email>,
) -> Result<Json<schemas::VerifiedEmail>, Error> {
    log::info!("Verifying: {}", &a.address);
    let res = check_syntax(&a.address);
    Ok(Json(schemas::VerifiedEmail::from(res)))
}

#[api_v2_operation(
summary = "Deep validation of a single email address",
description = "Returns a JSON object containing information on validity of email address, and the components of that address.",
operation_id = "Deep validate email address",
consumes = "application/json",
produces = "application/json"
)]
async fn deep_validate_address(
    o: oauth2::OAuth2Access,
    s: oauth2::EmailValidationScopeAccess,
    a: Json<schemas::Email>,
) -> Result<Json<schemas::VerifiedEmail>, Error> {
    log::info!("Verifying: {}", &a.address);
    let res = check_syntax(&a.address);
    Ok(Json(schemas::VerifiedEmail::from(res)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let spec = oas::build_spec(&VERSION, &GATEWAY);

    log::info!("Starting up on {}", gethostname().into_string().unwrap());
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number!");
    HttpServer::new(move || {
        App::new()
            .wrap_api_with_spec(spec.clone())
            .with_json_spec_v3_at("/spec/v3")
            .with_json_spec_at("/spec/v2")
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(web::resource("/v1/validate").route(web::post().to(validate_address)))
            .service(web::resource("/v1/deep_validate").route(web::post().to(deep_validate_address)))
            .build()
    })
    .bind_openssl(("0.0.0.0", port), builder)?
    .run()
    .await
}
