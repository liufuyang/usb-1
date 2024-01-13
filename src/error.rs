use std::fmt;

#[derive(Debug)]
pub enum LibError {
    CheckSumError(String),
    // ParseError(String),
}

impl std::error::Error for LibError {}

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibError::CheckSumError(x) => write!(f, "Check Error: {}, ", x),
            // LibError::ParseError(x) => write!(f, "Parse Error: {}", x),
        }
    }
}
