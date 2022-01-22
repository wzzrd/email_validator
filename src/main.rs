use std::time::Duration;
use actix_web::{App, Error, HttpResponse, HttpServer};
use actix_web::web;
use actix_web::middleware::Logger;
use check_if_email_exists::{check_email, CheckEmailInput};
use check_if_email_exists::syntax::check_syntax;
use serde::Deserialize;
use gethostname::gethostname;
use log;

#[derive(Deserialize)]
struct Email {
    address: String,
}

async fn validate_address(a: web::Json<Email>) -> Result<HttpResponse, Error> {
    log::info!("Verifying: {}", &a.address);
    // let mut input = CheckEmailInput::new(vec![a.address.clone()]);
    // input
    //     .set_from_email("maxim@wzzrd.com".into())
    //     .set_smtp_port(587)
    //     .set_smtp_timeout(Duration::new(1,0))
    //     .set_yahoo_use_api(false)
    //     .set_hello_name(gethostname().into_string().unwrap());
    // let res = check_email(&input).await;
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
            .service(web::resource("/v1/validate").route(web::get().to(validate_address)))
            .wrap(Logger::default())
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    
}
