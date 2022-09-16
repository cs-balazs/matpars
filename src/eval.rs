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
                Err(EvaluationError(format!(
                    "Value for variable '{}' not found",
                    var
                )))
            }
        }
        Type::Operator(operator) => match operator.operation {
            Operation::Unary(_) => todo!(),
            Operation::Binary(op) => match (tree.left.as_ref(), tree.right.as_ref()) {
                (None, None) => Err(EvaluationError(format!(
                    "Evaluation failed, a '{}' operation has no operands.",
                    operator.symbol
                ))),
                (None, _) => Err(EvaluationError(format!(
                    "Evaluation failed, a '{}' operation has no left operand.",
                    operator.symbol
                ))),
                (_, None) => Err(EvaluationError(format!(
                    "Evaluation failed, a '{}' operation has no right operand.",
                    operator.symbol
                ))),
                (Some(ref left), Some(ref right)) => match eval_tree(left, values) {
                    Err(e) => Err(e),
                    Ok(evaled_left) => match eval_tree(right, values) {
                        Err(e) => Err(e),
                        Ok(evaled_right) => Ok(op(evaled_left, evaled_right)),
                    },
                },
            },
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
