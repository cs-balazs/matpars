use crate::{EvaluationError, Operation, Tree, Type};
use std::collections::HashMap;

#[macro_export]
macro_rules! values {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert(String::from($key), $val); )*
        map
    }}
}

pub fn eval_tree(tree: &Tree, values: &HashMap<String, f64>) -> Result<f64, EvaluationError> {
    match &tree.node_type {
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
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn plus() {
        let parsed = parse("10 + x");

        let y = parsed.eval_for(values!["x" => 10.0f64]).unwrap();
        assert_eq!(y, 20.0f64);

        let y = parsed.eval_for(values!["x" => 4.0f64]).unwrap();
        assert_eq!(y, 14.0f64);
    }

    #[test]
    fn minus() {
        let parsed = parse("100 - x - 10");

        let y = parsed.eval_for(values!["x" => 20.0f64]).unwrap();
        assert_eq!(y, 70.0f64);

        let y = parsed.eval_for(values!["x" => 1.0f64]).unwrap();
        assert_eq!(y, 89.0f64);
    }

    #[test]
    fn times() {
        let parsed = parse("4 * x");

        let y = parsed.eval_for(values!["x" => 2.0f64]).unwrap();
        assert_eq!(y, 8.0f64);

        let y = parsed.eval_for(values!["x" => 110.0f64]).unwrap();
        assert_eq!(y, 440.0f64);
    }

    #[test]
    fn division() {
        let parsed = parse("x / 10");

        let y = parsed.eval_for(values!["x" => 100.0f64]).unwrap();
        assert_eq!(y, 10.0f64);
    }

    #[test]
    fn power() {
        let parsed = parse("2 ^ x");

        let y = parsed.eval_for(values!["x" => 2.0f64]).unwrap();
        assert_eq!(y, 4.0f64);

        let y = parsed.eval_for(values!["x" => 4.0f64]).unwrap();
        assert_eq!(y, 16.0f64);
    }
}
