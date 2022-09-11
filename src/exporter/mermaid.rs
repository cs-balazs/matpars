use super::FileExporter;
use crate::parser::{Operator, Tree, Type};
use std::{fs::File, io::Write};

pub const MERMAID_HTML_PREFIX: &str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\" /><title>Tree export</title></head><body><script src=\"https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js\"></script><div class=\"mermaid\">\ngraph TD\n";
pub const MERMAID_HTML_SUFFIX: &str = "</div></body></html>";

pub struct MermaidExporter;

impl FileExporter for MermaidExporter {
    fn export(parsed: &Tree, filename: &str) {
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
    formula: &Tree,
    parent: Option<&Tree>,
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

#[cfg(test)]
mod tests {
    use crate::{
        exporter::{
            mermaid::{MERMAID_HTML_PREFIX, MERMAID_HTML_SUFFIX},
            FileExporter, MermaidExporter,
        },
        parser::{Parser, TreeParser},
    };
    use std::fs;

    fn test_formula(formula: &str, filename: &str, mermaid: &str) {
        let parsed = TreeParser::parse(formula);
        MermaidExporter::export(&parsed.tree, filename);

        let file = fs::read(format!("{}.html", filename));

        fs::remove_file(format!("./{}.html", filename)).unwrap();

        if let Ok(content) = file {
            let content = content.iter().map(|byte| *byte as char).collect::<String>();
            let should_equal: String =
                MERMAID_HTML_PREFIX.to_string() + mermaid + MERMAID_HTML_SUFFIX;

            assert_eq!(content, should_equal);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn more_complex_tree() {
        test_formula(
            "x^2 + 3*x + (8*x - 21*9) + 5",
            "mermaid_test_1",
            "   A16((+)) --- A14((+))\n   A14((+)) --- A6((+))\n   A6((+)) --- A2((^))\n   A2((^)) --- A0((x))\n   A2((^)) --- A1((2))\n   A6((+)) --- A5((*))\n   A5((*)) --- A3((3))\n   A5((*)) --- A4((x))\n   A14((+)) --- A13((\"-\"))\n   A13((\"-\")) --- A9((*))\n   A9((*)) --- A7((8))\n   A9((*)) --- A8((x))\n   A13((\"-\")) --- A12((*))\n   A12((*)) --- A10((21))\n   A12((*)) --- A11((9))\n   A16((+)) --- A15((5))\n",
        );
    }

    #[test]
    fn simple_tree() {
        test_formula(
            "x + 12",
            "mermaid_test_2",
            "   A2((+)) --- A0((x))\n   A2((+)) --- A1((12))\n",
        );
    }
}
