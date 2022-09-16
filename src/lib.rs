mod errors;
mod eval;
pub mod exporter;
mod parse;

use errors::{EvaluationError, InvalidVariableError};
use eval::eval_tree;
pub use parse::parse;
use std::{cell::Cell, collections::HashMap};

#[derive(Debug)]
pub struct Operator {
    pub symbol: String,
    pub operation: Operation,
}

impl Operator {
    fn new(symbol: String, operation: Operation) -> Operator {
        Operator { symbol, operation }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Operation {
    Unary(fn(f64) -> f64),
    Binary(fn(f64, f64) -> f64),
}

thread_local!(static NODE_ID: Cell<usize> = Cell::new(0));

#[derive(Debug)]
pub enum Type {
    Operator(Operator),
    Constant(f64),
    Variable(String),
}

#[derive(Debug)]
pub struct Tree {
    pub id: usize,
    pub node_type: Type,
    pub left: Box<Option<Tree>>,
    pub right: Box<Option<Tree>>,
}

#[derive(Debug)]
pub struct Matpars {
    pub variables: HashMap<String, f64>,
    pub tree: Tree,
}

impl Matpars {
    pub fn set_variable(&mut self, name: &str, value: f64) -> Result<(), InvalidVariableError> {
        if self.variables.insert(name.to_string(), value).is_some() {
            Ok(())
        } else {
            Err(InvalidVariableError(format!(
                "Cannot set variable '{}', expected one of: {:?}",
                name,
                self.variables.keys().collect::<Vec<&String>>()
            )))
        }
    }
    pub fn eval(&self) -> Result<f64, EvaluationError> {
        eval_tree(&self.tree, &self.variables)
    }
    pub fn eval_for(&self, values: HashMap<String, f64>) -> Result<f64, EvaluationError> {
        eval_tree(&self.tree, &values)
    }
    pub fn find_root(&self) -> f64 {
        todo!()
    }
}

impl Tree {
    fn new(node_type: Type, left: Option<Tree>, right: Option<Tree>) -> Tree {
        NODE_ID.with(|thread_id| Tree {
            id: {
                let id = thread_id.get();
                thread_id.set(id + 1);
                id
            },
            node_type,
            left: Box::new(left),
            right: Box::new(right),
        })
    }
}
