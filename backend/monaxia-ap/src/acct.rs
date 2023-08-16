use monaxia_data::user::{validate_username_format, UsernameError};
use thiserror::Error as ThisError;
use url::Url;

pub const ACCT_USERNAME_LIMIT: usize = 64;

pub struct Acct {
    username: String,
    origin: String,
}

#[derive(Debug, Clone, ThisError)]
pub enum AcctError {
    #[error("origin is invalid")]
    InvalidOrigin,

    #[error("username is invalid ({0})")]
    Username(#[from] UsernameError),

    #[error("invalid formatted")]
    InvalidFormat,
}

impl Acct {
    pub fn new(username: &str, origin: &str) -> Result<Acct, AcctError> {
        validate_username_format(username, 1..=ACCT_USERNAME_LIMIT)?;

        let origin_url = format!("https://{origin}");
        let Ok(_) = Url::parse(&origin_url) else {
            return Err(AcctError::InvalidOrigin);
        };

        Ok(Acct {
            username: username.to_string(),
            origin: origin.to_string(),
        })
    }

    pub fn parse(input: &str) -> Result<Acct, AcctError> {
        let stripped = input
            .strip_prefix("acct:")
            .or(input.strip_prefix('@'))
            .unwrap_or(input);
        let mut parts = stripped.split('@');
        let username = parts.next().ok_or(AcctError::InvalidFormat)?;
        let origin = parts.next().ok_or(AcctError::InvalidFormat)?;
        let None = parts.next() else { return Err(AcctError::InvalidFormat) };

        Acct::new(username, origin)
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    pub fn to_subject(&self) -> String {
        format!("acct:{}@{}", self.username, self.origin)
    }
}
