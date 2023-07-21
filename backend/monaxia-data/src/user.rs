use std::ops::RangeInclusive;

use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error as ThisError;

static RE_USERNAME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^[A-Za-z0-9_]+$"#).expect("invalid regex"));

/// Represents username format error.
/// Not covered for in-use check or prohibited patterns.
#[derive(Debug, Clone, ThisError)]
pub enum UsernameError {
    #[error("out of length range; it should be in {0:?}")]
    OutOfLength(RangeInclusive<usize>),

    #[error("non-ASCII character is prohibited")]
    NonAscii,

    #[error("only alphanumeric and underscore is allowed")]
    Inencodable,

    #[error("other error")]
    Other,
}

/// Validates username format.
pub fn validate_username_format(
    input: &str,
    length_range: RangeInclusive<usize>,
) -> Result<(), UsernameError> {
    if !length_range.contains(&input.len()) {
        return Err(UsernameError::OutOfLength(length_range));
    }

    if !input.is_ascii() {
        return Err(UsernameError::NonAscii);
    }

    if !RE_USERNAME.is_match(input) {
        return Err(UsernameError::Inencodable);
    }

    Ok(())
}
