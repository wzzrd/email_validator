use actix_web::{App, Error, HttpResponse, HttpServer};
use actix_web::web;
use actix_web::middleware::Logger;
use check_if_email_exists::{check_email, CheckEmailInput};
use log;

async fn validate_address(a: &str) -> Result<HttpResponse, Error> {
    let input = CheckEmailInput::new(vec![a.into()]);
    let res = check_email(&input).await;
    Ok(HttpResponse::Ok().json(res))
}


#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    log::info!("Starting up!");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/v1/validate", web::get().to(validate_address))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    
}
