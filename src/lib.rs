pub mod exporter;

// Should be the maximum the values in the weights map + 1 to outpower everything outside the brackets
const BRACKETS_EXTRA_WEIGHT: u32 = 4;

use std::{
    cell::Cell,
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Division,
    Power,
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
            Err(InvalidVariableError::new(
                format!(
                    "Cannot set variable '{}', expected one of: {:?}",
                    name,
                    self.variables.keys().collect::<Vec<&String>>()
                )
                .as_str(),
            ))
        }
    }
    pub fn eval(&self) -> Result<f64, EvaluationError> {
        eval_tree(&self.tree, &self.variables)
    }
    pub fn eval_for(&self, values: HashMap<String, f64>) -> Result<f64, EvaluationError> {
        eval_tree(&self.tree, &values)
    }
}

pub fn parse(input: &str) -> Matpars {
    // TODO: Somehow construct this as a const
    let weights: HashMap<char, u32> =
        HashMap::from([('+', 1), ('-', 1), ('*', 2), ('/', 2), ('^', 3)]);

    let mut input_copy = input.trim().replace(' ', "");
    let mut extra_weight: u32 = 0;
    let mut weights: Vec<u32> = input_copy
        .clone()
        .chars()
        .map(|chr| {
            if let Some(val) = weights.get(&chr) {
                *val + extra_weight
            } else {
                if chr == '(' {
                    extra_weight += BRACKETS_EXTRA_WEIGHT
                } else if chr == ')' {
                    extra_weight -= BRACKETS_EXTRA_WEIGHT
                };
                0
            }
        })
        .collect::<Vec<u32>>();

    while let Some(index) = input_copy.find(|c: char| (c == '(') || (c == ')')) {
        input_copy.remove(index);
        weights.remove(index);
    }

    let tree = construct_tree(&input_copy, weights.as_slice());

    let mut variables: HashMap<String, f64> = HashMap::new();
    collect_variables(&tree, &mut variables);

    Matpars { tree, variables }
}

fn operator_to_symbol(operator: &Operator) -> String {
    let symbols: HashMap<Operator, String> = HashMap::from([(Operator::Plus, String::from("+"))]);

    symbols.get(operator).unwrap().to_string()
}

fn check_operands(tree: &Tree, operator: &Operator) -> Result<(), EvaluationError> {
    if tree.left.is_none() {
        return Err(EvaluationError::new(
            format!(
                "Evaluation failed, a '{}' operation has no left operand.",
                operator_to_symbol(operator)
            )
            .as_str(),
        ));
    }
    if tree.right.is_none() {
        return Err(EvaluationError::new(
            format!(
                "Evaluation failed, a '{}' operation has no right operand.",
                operator_to_symbol(operator)
            )
            .as_str(),
        ));
    }
    Ok(())
}

fn eval_sides(
    tree: &Tree,
    values: &HashMap<String, f64>,
    operation: fn(f64, f64) -> f64,
) -> Result<f64, EvaluationError> {
    let left = eval_tree((*tree.left).as_ref().unwrap(), values);
    if let Err(e) = left {
        return Err(e);
    }
    let right = eval_tree((*tree.right).as_ref().unwrap(), values);
    if let Err(e) = right {
        return Err(e);
    }
    Ok(operation(left.unwrap(), right.unwrap()))
}

fn eval_tree(tree: &Tree, values: &HashMap<String, f64>) -> Result<f64, EvaluationError> {
    let option: Result<f64, EvaluationError> = match &tree.node_type {
        Type::Constant(val) => Ok(*val),
        Type::Variable(var) => {
            if let Some(val) = values.get(var) {
                Ok(*val)
            } else {
                Err(EvaluationError::new(
                    format!("Value for variable '{}' not found", var).as_str(),
                ))
            }
        }
        Type::Operator(operator) => match operator {
            Operator::Plus => {
                if let Err(e) = check_operands(tree, operator) {
                    Err(e)
                } else {
                    eval_sides(tree, values, |a, b| a + b)
                }
            }
            Operator::Minus => {
                if let Err(e) = check_operands(tree, operator) {
                    Err(e)
                } else {
                    eval_sides(tree, values, |a, b| a - b)
                }
            }
            Operator::Times => {
                if let Err(e) = check_operands(tree, operator) {
                    Err(e)
                } else {
                    eval_sides(tree, values, |a, b| a * b)
                }
            }
            Operator::Division => {
                if let Err(e) = check_operands(tree, operator) {
                    Err(e)
                } else {
                    eval_sides(tree, values, |a, b| a / b)
                }
            }
            Operator::Power => {
                if let Err(e) = check_operands(tree, operator) {
                    Err(e)
                } else {
                    eval_sides(tree, values, f64::powf)
                }
            }
        },
    };

    option
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

pub trait Parser {
    fn parse(input: &str) -> Matpars;
}

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
    fn new(message: &str) -> InvalidVariableError {
        InvalidVariableError {
            message: message.to_string(),
        }
    }
}

impl EvaluationError {
    fn new(message: &str) -> EvaluationError {
        EvaluationError {
            message: message.to_string(),
        }
    }
}

impl Error for InvalidVariableError {}
impl Error for EvaluationError {}

fn construct_tree(input: &String, weights: &[u32]) -> Tree {
    let min_weight = {
        let mut minimum = u32::MAX;
        for &weight in weights {
            if weight > 0 && weight < minimum {
                minimum = weight;
            };
        }
        if minimum == u32::MAX {
            minimum = 0;
        }
        minimum
    };

    if min_weight == 0 {
        if input.chars().all(char::is_alphabetic) {
            return Tree::new(
                Type::Variable(input.to_string()),
                Option::None,
                Option::None,
            );
        };
        if input.chars().all(char::is_numeric) {
            return Tree::new(
                Type::Constant(input.parse::<f64>().unwrap()),
                Option::None,
                Option::None,
            );
        };
    };

    let rightmost_min_weight_index =
        weights.iter().enumerate().fold(
            0,
            |acc, curr| if *curr.1 == min_weight { curr.0 } else { acc },
        );
    let left = input[0..rightmost_min_weight_index].to_string();
    let right = input[rightmost_min_weight_index + 1..].to_string();
    let left_weights = &weights[0..rightmost_min_weight_index];
    let right_weights = &weights[rightmost_min_weight_index + 1..];

    Tree::new(
        Type::Operator(
            match input.chars().nth(rightmost_min_weight_index).unwrap() {
                '+' => Operator::Plus,
                '-' => Operator::Minus,
                '*' => Operator::Times,
                '/' => Operator::Division,
                '^' => Operator::Power,
                _ => panic!(),
            },
        ),
        Some(construct_tree(&left, left_weights)),
        Some(construct_tree(&right, right_weights)),
    )
}

fn collect_variables(tree: &Tree, variables: &mut HashMap<String, f64>) {
    if let Type::Variable(var) = &tree.node_type {
        variables.insert(var.to_string(), 0.0f64);
    }

    if let Some(left) = &*tree.left {
        collect_variables(left, variables);
    }

    if let Some(right) = &*tree.right {
        collect_variables(right, variables);
    }
}
