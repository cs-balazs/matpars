use crate::{EvaluationError, Operator, Tree, Type};
use std::collections::HashMap;

fn check_operands(tree: &Tree, operator: &Operator) -> Result<(), EvaluationError> {
    if tree.left.is_none() {
        return Err(EvaluationError::new(format!(
            "Evaluation failed, a '{}' operation has no left operand.",
            operator_to_symbol(operator)
        )));
    }
    if tree.right.is_none() {
        return Err(EvaluationError::new(format!(
            "Evaluation failed, a '{}' operation has no right operand.",
            operator_to_symbol(operator)
        )));
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

pub fn eval_tree(tree: &Tree, values: &HashMap<String, f64>) -> Result<f64, EvaluationError> {
    let option: Result<f64, EvaluationError> = match &tree.node_type {
        Type::Constant(val) => Ok(*val),
        Type::Variable(var) => {
            if let Some(val) = values.get(var) {
                Ok(*val)
            } else {
                Err(EvaluationError::new(format!(
                    "Value for variable '{}' not found",
                    var
                )))
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

fn operator_to_symbol(operator: &Operator) -> String {
    HashMap::from([
        (Operator::Plus, String::from("+")),
        (Operator::Minus, String::from("-")),
        (Operator::Times, String::from("*")),
        (Operator::Division, String::from("/")),
        (Operator::Power, String::from("^")),
    ])
    .get(operator)
    .unwrap()
    .to_string()
}
