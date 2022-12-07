use crate::Apiv2Security;
use crate::Deserialize;
use crate::Error;
use crate::FromRequest;
use crate::HttpRequest;
use crate::Ready;
use crate::{ready, GATEWAY};
use actix_web::error::ErrorUnauthorized;
use log::debug;

// The code below is based on macros expansions of the #[openapi] and #Apiv2Security macros
// Using those macros limits the run time customization options for token_url, which is
// not desirable for this app
#[derive(Deserialize, Debug)]
pub struct OAuth2Access;
impl paperclip::v2::schema::Apiv2Schema for OAuth2Access {
    fn name() -> Option<String> {
        Some("OAuth2 Authorization".to_string())
    }
    fn security_scheme() -> Option<paperclip::v2::models::SecurityScheme> {
        let token_url = format!("https://{}/oauth2/token", &GATEWAY);
        Some(paperclip::v2::models::SecurityScheme {
            type_: "oauth2".to_string(),
            name: None,
            in_: None,
            flow: Some("application".to_string()),
            auth_url: None,
            token_url: Some(token_url),
            scopes: std::collections::BTreeMap::new(),
            description: Some("Allow or prevent access to this valuable API".to_string()),
        })
    }
}
impl paperclip::actix::OperationModifier for OAuth2Access {}

// Checks whether the x-consumer-username header is set, and the anonymous consumer header is not
impl FromRequest for OAuth2Access {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(h: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let headers = h.headers();
        if !headers.contains_key("X-Anonymous-Consumer")
            && headers.contains_key("X-Consumer-Username")
        {
            return ready(Ok(Self {}));
        }
        let error = ErrorUnauthorized("User not logged in");
        return ready(Err(error));
    }
}

#[derive(Apiv2Security, Deserialize, Debug)]
#[openapi(parent = "OAuth2Access", scopes("email_validation"))]
pub struct EmailValidationScopeAccess;

// Checks whether the x-authenticated-scope Kong header includes "email_validation"
impl FromRequest for EmailValidationScopeAccess {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(h: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let headers = h.headers();
        debug!("headers: {:?}", headers);
        if headers.contains_key("X-Authenticated-Scope") {
            let scopes = headers
                .get("X-Authenticated-Scope")
                .expect("Can't read X-Authenticated-Scope header value")
                .to_str()
                .expect("Can't convert X-Authenticated-Scope header value to string");
            let s = scopes.split(",");
            let scope_list = s.collect::<Vec<&str>>();
            if scope_list.contains(&"email_validation") {
                return ready(Ok(Self {}));
            }
        }
        let error = ErrorUnauthorized("No valid scope found");
        return ready(Err(error));
    }
}
