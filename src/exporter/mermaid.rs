use super::FileExporter;
use crate::parser::{Matpars, Operator, Type};
use std::{fs::File, io::Write};

pub struct MermaidParser;

impl FileExporter for MermaidParser {
    fn export(parsed: &Matpars, filename: &str) {
        let mut mermaid = String::from("");

        let html:String = String::from("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\" /><title>matpars export</title></head><body><script src=\"https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js\"></script><div class=\"mermaid\">\ngraph TD\n") + to_mermaid(parsed, Option::None, &mut mermaid).as_str() + "</div></body></html>";

        let mut file = File::create(format!("{}.html", filename)).unwrap();
        Write::write_all(&mut file, html.as_bytes()).unwrap();
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
            match &parent.node_type {
                Type::Operator(operator) => match operator {
                    Operator::Minus => String::from("\"-\""),
                    Operator::Plus => String::from("+"),
                    Operator::Times => String::from("*"),
                    Operator::Division => String::from("/"),
                    Operator::Power => String::from("^"),
                },
                Type::Constant(constant) => constant.to_string(),
                Type::Variable(var) => String::from(var),
            },
            formula.id,
            match &formula.node_type {
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
