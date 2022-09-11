use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct InvalidVariableError {
    message: String,
}

#[derive(Debug)]
pub struct EvaluationError {
    message: String,
}

impl Display for InvalidVariableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)
    }
}
impl Display for EvaluationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl InvalidVariableError {
    pub fn new(message: &str) -> InvalidVariableError {
        InvalidVariableError {
            message: message.to_string(),
        }
    }
}

impl EvaluationError {
    pub fn new(message: &str) -> EvaluationError {
        EvaluationError {
            message: message.to_string(),
        }
    }
}

impl Error for InvalidVariableError {}
impl Error for EvaluationError {}
