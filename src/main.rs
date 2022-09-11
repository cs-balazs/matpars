use std::collections::HashMap;

use matpars::{
    exporter::{self, FileExporter},
    parser::{self, Parser},
};

fn main() {
    let mut parsed =
        parser::TreeParser::parse("3* z - x^2 + 3*x + (8*x - 21*9) + 5 / ( 10 * a - 3 ) + x");
    exporter::MermaidExporter::export(&parsed.tree, "mermaid");

    println!("{:?}", parsed.variables);

    parsed.set_variable("x", 10.0f64).unwrap();

    println!("{:?}", parsed.variables);

    parsed.set_variable("x", 1.1653414f64).unwrap();

    println!("{:?}", parsed.variables);

    println!("{}", parsed.eval().unwrap());

    println!(
        "{}",
        parsed
            .eval_for(HashMap::from([
                (String::from("x"), 1.1653414f64),
                (String::from("z"), 0.0f64),
                (String::from("a"), 0.0f64)
            ]))
            .unwrap()
    );
}
