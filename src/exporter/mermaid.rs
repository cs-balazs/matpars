use std::{fs::File, io::Write};

use crate::parser::{Formula, Node, Operator};

pub fn export(formula: Formula) {
    let mut mermaid = String::from("");

    let html:String = String::from("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\" /><title>matpars export</title></head><body><script src=\"https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js\"></script><div class=\"mermaid\">\ngraph TD\n") + to_mermaid(&formula, Option::None, &mut mermaid).as_str() + "</div></body></html>";

    let mut file = File::create("export.html").unwrap();
    Write::write_all(&mut file, html.as_bytes()).unwrap();
}

fn to_mermaid<'a>(
    formula: &Formula,
    parent: Option<&Formula>,
    mermaid: &'a mut String,
) -> &'a mut String {
    if let Some(parent) = parent {
        *mermaid += format!(
            "   A{}(({})) --- A{}(({}))\n",
            parent.id,
            match &parent.node {
                Node::Operator(operator) => match operator {
                    Operator::Minus => String::from("\"-\""),
                    Operator::Plus => String::from("+"),
                    Operator::Times => String::from("*"),
                    Operator::Division => String::from("/"),
                    Operator::Power => String::from("^"),
                },
                Node::Constant(constant) => constant.to_string(),
                Node::Variable(var) => String::from(var),
            },
            formula.id,
            match &formula.node {
                Node::Operator(operator) => match operator {
                    Operator::Minus => String::from("\"-\""),
                    Operator::Plus => String::from("+"),
                    Operator::Times => String::from("*"),
                    Operator::Division => String::from("/"),
                    Operator::Power => String::from("^"),
                },
                Node::Constant(constant) => constant.to_string(),
                Node::Variable(var) => String::from(var),
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
