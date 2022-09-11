use crate::{Matpars, Tree, Type};

use super::StringExporter;

pub struct PolishExporter;

impl StringExporter for PolishExporter {
    fn to_string(parsed: &Matpars) -> String {
        let mut polish = String::new();
        to_polish(&parsed.tree, &mut polish).to_string()
    }
}

fn to_polish<'a>(parsed: &Tree, polish: &'a mut String) -> &'a mut String {
    *polish += format!("{} ", {
        match &parsed.node_type {
            Type::Constant(val) => val.to_string(),
            Type::Variable(var) => var.to_string(),
            Type::Operator(op) => op.symbol.to_string(),
        }
    })
    .as_str();

    if let Some(ref left) = *parsed.left {
        to_polish(left, polish);
    }

    if let Some(ref right) = *parsed.right {
        to_polish(right, polish);
    }

    polish
}
