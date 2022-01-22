use actix_web::{App, Error, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use paperclip::actix::{
    OpenApiExt, Apiv2Schema, api_v2_operation,
    web::{self, Json},
};
use check_if_email_exists::syntax::check_syntax;
use serde::Deserialize;
use gethostname::gethostname;
use log;

#[derive(Deserialize, Apiv2Schema)]
struct Email {
    address: String,
}

#[api_v2_operation]
async fn validate_address(a: web::Json<Email>) -> Result<HttpResponse, Error> {
    log::info!("Verifying: {}", &a.address);
    let res = check_syntax(&a.address);
    Ok(HttpResponse::Ok().json(res))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    log::info!("Starting up on {}", gethostname().into_string().unwrap());
    HttpServer::new(move || {
        App::new()
            .wrap_api()
            .service(web::resource("/v1/validate").route(web::get().to(validate_address)))
            .with_json_spec_at("/spec/v1")
            .wrap(Logger::default())
            .build()
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    
}
