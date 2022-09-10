use super::FileExporter;
use crate::parser::{Matpars, Operator, Type};
use std::{fs::File, io::Write};

const MERMAID_HTML_PREFIX: &str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\" /><title>matpars export</title></head><body><script src=\"https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js\"></script><div class=\"mermaid\">\ngraph TD\n";
const MERMAID_HTML_SUFFIX: &str = "</div></body></html>";

pub struct MermaidExporter;

impl FileExporter for MermaidExporter {
    fn export(parsed: &Matpars, filename: &str) {
        let mut mermaid = String::from("");

        let html: String = MERMAID_HTML_PREFIX.to_string()
            + to_mermaid(parsed, Option::None, &mut mermaid).as_str()
            + MERMAID_HTML_SUFFIX;

        let mut file = File::create(format!("{}.html", filename)).unwrap();
        Write::write_all(&mut file, html.as_bytes()).unwrap();
    }
}

fn type_to_label(node_type: &Type) -> String {
    match node_type {
        Type::Operator(operator) => match operator {
            Operator::Minus => String::from("\"-\""),
            Operator::Plus => String::from("+"),
            Operator::Times => String::from("*"),
            Operator::Division => String::from("/"),
            Operator::Power => String::from("^"),
        },
        Type::Constant(constant) => constant.to_string(),
        Type::Variable(var) => String::from(var),
    }
}

fn to_mermaid<'a>(
    formula: &Matpars,
    parent: Option<&Matpars>,
    mermaid: &'a mut String,
) -> &'a mut String {
    if let Some(parent) = parent {
        *mermaid += format!(
            "   A{}(({})) --- A{}(({}))\n",
            parent.id,
            type_to_label(&parent.node_type),
            formula.id,
            type_to_label(&formula.node_type)
        )
        .as_str();
    }

    if let Some(ref left) = *formula.left {
        to_mermaid(left, Some(formula), mermaid);
    };

    if let Some(ref right) = *formula.right {
        to_mermaid(right, Some(formula), mermaid);
    };

    mermaid
}
