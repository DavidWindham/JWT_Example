use std::{error, fmt};

type Result<T> = std::result::Result<T, TokenError>;

#[derive(Debug, Clone)]
pub enum TokenError {
    ExpiredToken,
    InvalidToken,
    InvalidHeaderAlgorithm,
    InvalidExpiry,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenError::ExpiredToken => write!(f, "token has expired"),
            TokenError::InvalidToken => write!(f, "token is invalid"),
            TokenError::InvalidHeaderAlgorithm => {
                write!(f, "token header contains incorrect or missing algorithm")
            }
            TokenError::InvalidExpiry => write!(f, "token is invalid"),
        }
    }
}

impl error::Error for TokenError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            TokenError::ExpiredToken => None,
            TokenError::InvalidToken => None,
            TokenError::InvalidHeaderAlgorithm => None,
            TokenError::InvalidExpiry => None,
        }
    }
}
