use crate::Apiv2Security;
use crate::Deserialize;
use crate::Error;
use crate::FromRequest;
use crate::HttpRequest;
use crate::Ready;
use crate::ready;

#[derive(Apiv2Security, Deserialize)]
#[openapi(
oauth2,
alias = "OAuth2 Authorization",
description = "Allow or prevent access to this valuable API",
token_url = "https://gw.wzzrd.com/oauth2/token",
// This is "application" because that is what Swagger 2.0 wants to see; after all, the
// paperclip library only translates v2 into v3. It doesn't natively do v3.
flow = "application"
)]
pub struct OAuth2Access;

// This just returns a Future containing Ok()?
// For now, this is enough, since oauth2 is handled at the gateway; this is just here for
// OAS building purposes
impl FromRequest for OAuth2Access {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(_: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        ready(Ok(Self {}))
    }
}

#[derive(Apiv2Security, Deserialize)]
#[openapi(parent = "OAuth2Access", scopes("email_validation", "dns_validation"))]
pub struct EmailValidationScopeAccess;

// This just returns a Future containing Ok()?
// For now, this is enough, since oauth2 is handled at the gateway; this is just here for
// OAS building purposes
impl FromRequest for EmailValidationScopeAccess {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(_: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        ready(Ok(Self {}))
    }
}
