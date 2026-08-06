use std::error::Error;
use std::fmt;
#[derive(Debug)]
#[non_exhaustive]
pub enum TypeError {
    Validation { message: String },
}

impl TypeError {
    #[inline]
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::Validation { message } => {
                write!(f, "validation error: {message}")
            }
        }
    }
}

impl Error for TypeError {}

pub type TypeResult<T> = Result<T, TypeError>;
