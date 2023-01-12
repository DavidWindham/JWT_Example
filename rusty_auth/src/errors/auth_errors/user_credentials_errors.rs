// use std::{error, fmt};

// type Result<T> = std::result::Result<T, UserCredentialsError>;

// #[derive(Debug, Clone)]
// pub enum UserLoginError {
//     MissingUsername,
//     MissingPassword,
//     NoUser
// }

// impl fmt::Display for UserCredentialsError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             UserCredentialsError:: => write!(f, ""),
//         }
//     }
// }

// impl error::Error for UserCredentialsError {
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         match *self {
//             UserCredentialsError:: => None,
//         }
//     }
// }
