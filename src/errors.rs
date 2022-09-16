use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct InvalidVariableError(pub String);

#[derive(Debug)]
pub struct EvaluationError(pub String);

impl Display for InvalidVariableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
impl Display for EvaluationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl Error for InvalidVariableError {}
impl Error for EvaluationError {}
