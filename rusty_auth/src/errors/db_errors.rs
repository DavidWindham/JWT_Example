use diesel::result::Error;
use std::{error, fmt};

type Result<T> = std::result::Result<T, DatabaseError>;

#[derive(Debug)]
pub enum DatabaseError {
    RegisterFailure(Box<dyn error::Error + Send + Sync>),
    LoginFailure(Box<dyn error::Error + Send + Sync>),
    CredentialsFailure(Box<dyn error::Error + Send + Sync>),
    Other(Box<dyn error::Error + Send + Sync>),
    Exception(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            DatabaseError::RegisterFailure(wrapped_error) => {
                write!(f, "unable to register user: {}", wrapped_error)
            }
            DatabaseError::LoginFailure(wrapped_error) => {
                write!(f, "unable to login user: {}", wrapped_error)
            }
            DatabaseError::CredentialsFailure(wrapped_error) => {
                write!(f, "unable to validate credentials: {}", wrapped_error)
            }
            DatabaseError::Other(wrapped_error) => write!(
                f,
                "error from database outside of handled cases: {}",
                wrapped_error
            ),
            DatabaseError::Exception(unknown_error_string) => {
                write!(
                    f,
                    "not sure if you should ever get this: {}",
                    unknown_error_string
                )
            }
        }
    }
}

impl error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DatabaseError::RegisterFailure(_) => None,
            DatabaseError::LoginFailure(_) => None,
            DatabaseError::CredentialsFailure(_) => None,
            DatabaseError::Other(_) => None,
            DatabaseError::Exception(_) => None,
        }
    }
}
