use std::{error, fmt};

type Result<T> = std::result::Result<T, TokenError>;

#[derive(Debug, Clone)]
pub enum TokenError {
    ExpiredToken,
    InvalidToken,
    InvalidHeaderAlgorithm,
    InvalidExpiry,
    UnableToSign,
    UnableToGetHmacKey,
    MissingBodyKey,
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
            TokenError::UnableToSign => write!(f, "unable to sign token"),
            TokenError::UnableToGetHmacKey => write!(f, "unable to get hmac key"),
            TokenError::MissingBodyKey => write!(f, "unable to find requested key"),
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
            TokenError::UnableToSign => None,
            TokenError::UnableToGetHmacKey => None,
            TokenError::MissingBodyKey => None,
        }
    }
}
