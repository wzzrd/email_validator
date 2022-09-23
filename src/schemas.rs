use crate::Apiv2Schema;
use crate::Deserialize;
use crate::Serialize;
use crate::SyntaxDetails;

#[derive(Deserialize, Apiv2Schema)]
/// The email address to be checked
pub struct Email {
    /// The email address as a string
    pub address: String,
}

#[derive(Serialize, Apiv2Schema)]
/// The verification results
pub struct VerifiedEmail {
    /// The supplied email address
    pub address: String,
    /// The domain part of the supplied email address
    pub domain: String,
    /// Boolean indicating whether the supplied email address is syntactically valid
    pub is_valid_syntax: bool,
    /// The username part of the supplied email address
    pub username: String,
}


impl From<SyntaxDetails> for VerifiedEmail {
    fn from(s: SyntaxDetails) -> Self {
        let address = match s.address {
            Some(a) => format!("{}", a),
            None => "invalid address".to_string(),
        };
        VerifiedEmail {
            address,
            domain: s.domain.into(),
            is_valid_syntax: s.is_valid_syntax.into(),
            username: s.username.into(),
        }
    }
}
