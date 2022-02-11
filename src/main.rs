#[macro_use] extern crate serde_derive;

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use actix_web::{App, Error, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use paperclip::actix::{
    OpenApiExt, Apiv2Schema, api_v2_operation,
    web::{self, Json},
};
use paperclip::api_v2_schema;
use check_if_email_exists::syntax::{check_syntax, SyntaxDetails};
use serde::{Deserialize, Serialize};
use gethostname::gethostname;
use indexmap::IndexMap;
use paperclip::v2::models::{Contact, DefaultApiRaw, Info, License};
use log;
use strum::Display;

#[derive(Deserialize, Apiv2Schema)]
/// The email address to be checked
struct Email {
    /// The email address as a string
    address: String,
}

#[derive(Serialize, Apiv2Schema)]
/// The verification results
struct VerifiedEmail {
    /// The supplied email address
    address: String,
    /// The domain part of the supplied email address
    domain: String,
    /// Boolean indicating whether the supplied email address is syntactically valid
    is_valid_syntax: bool,
    /// The username part of the supplied email address
    username: String,
}

impl From<SyntaxDetails> for VerifiedEmail {
    fn from(s: SyntaxDetails) -> Self {
        let address = match s.address {
            Some(a) => format!("{}", a),
            None => "invalid address".to_string(),
        };
        VerifiedEmail {
            address: address,
            domain: s.domain.into(),
            is_valid_syntax: s.is_valid_syntax.into(),
            username: s.username.into()
        }
    }
}

#[derive(Debug, Display, Serialize)]
#[strum(serialize_all = "lowercase")]
enum version_lifecycle {
    ACTIVE,
    DEPRECATED,
    DRAFT
}

// impl fmt::Display for version_lifecycle {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

/// Validate the email address
///
/// Will provide information syntax validity, and split the address into domain and username parts
#[api_v2_operation]
async fn validate_address(a: web::Json<Email>) -> Result<web::Json<VerifiedEmail>, Error> {
    log::info!("Verifying: {}", &a.address);
    let res = check_syntax(&a.address);
    Ok(web::Json(VerifiedEmail::from(res)))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    log::info!("Setting schema defaults");
    let mut spec = DefaultApiRaw::default();
    let badges = serde_json::json!([{"name": "env", "value": "dev"}, {"name": "security", "value": "medium"}, { "name": "region", "value": "global"}]);
    let mut info_exts = BTreeMap::new();
    info_exts.insert("x-category".to_string(), serde_json::json!("Utility APIs"));
    info_exts.insert("x-long-description".to_string(), serde_json::Value::String(include_str!("../README.md").to_string()));
    info_exts.insert("x-thumbnail".to_string(), serde_json::Value::String("https://en.gravatar.com/userimage/3149428/abb6f0635c488a6833a4966c9cff4ea2.jpeg".to_string()));
    info_exts.insert("x-version-lifecycle".to_string(), serde_json::Value::String(version_lifecycle::ACTIVE.to_string()));
    info_exts.insert("x-collections".to_string(), serde_json::Value::Array(vec![serde_json::Value::String("consumer-onboarding".to_string())]));
    info_exts.insert("x-website".to_string(), serde_json::Value::String("https://www.wzzrd.com".to_string()));
    info_exts.insert("x-public".to_string(), serde_json::Value::Bool(false));
    info_exts.insert("termsOfService".to_string(), serde_json::Value::String("https:///www.wzzrd.com/tos".to_string()));
    info_exts.insert("x-badges".to_string(), badges);

    let mut root_exts = BTreeMap::new();
    root_exts.insert("x-documentation".to_string(),
                     serde_json::json!({"readme": "this is the readme string", "spotlights": [{ "title": "a spotlight", "description": "the spotlight explained", "link": "https://www.wzzrd.com"}]}));

    spec.extensions = root_exts;

    spec.info = Info {
        version: "0.2".into(),
        contact: Some(Contact { name: Some("Maxim Burgerhout".to_string()), email: Some("maxim@wzzrd.com".to_string()), url: Some("https://www.wzzrd.com".to_string()) }),
        license: Some(License { name: Some("something legal".to_string()), url: Some("https://www.wzzrd.com".to_string()) }),
        title: "Email address verification".into(),
        description: Some("This API verifies the validity of email addresses".to_string()),
        extensions: info_exts,
    };

    log::info!("Starting up on {}", gethostname().into_string().unwrap());
    HttpServer::new(move || {
        App::new()
            .wrap_api_with_spec(spec.clone())
            .with_json_spec_at("/spec/v2")
            .wrap(Logger::default())
            .service(web::resource("/v1/validate").route(web::post().to(validate_address)))
            .build()
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    
}
