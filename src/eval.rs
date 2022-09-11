use crate::{EvaluationError, Operation, Tree, Type};
use std::collections::HashMap;

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
        Type::Operator(operator) => match operator.operation {
            Operation::Unary(_) => todo!(),
            Operation::Binary(op) => {
                if tree.left.is_none() {
                    return Err(EvaluationError::new(format!(
                        "Evaluation failed, a '{}' operation has no left operand.",
                        operator.symbol
                    )));
                }

                if tree.right.is_none() {
                    return Err(EvaluationError::new(format!(
                        "Evaluation failed, a '{}' operation has no right operand.",
                        operator.symbol
                    )));
                }

                let left = eval_tree((*tree.left).as_ref().unwrap(), values);
                if let Err(e) = left {
                    return Err(e);
                }
                let right = eval_tree((*tree.right).as_ref().unwrap(), values);
                if let Err(e) = right {
                    return Err(e);
                }
                Ok(op(left.unwrap(), right.unwrap()))
            }
        },
    };

    option
}
