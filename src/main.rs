use matpars::{
    self,
    exporter::{self, FileExporter},
};
use std::collections::HashMap;

fn main() {
    let mut parsed = matpars::parse("3* z - x^2 + 3*x + (8*x - 21*9) + 5 / ( 10 * a - 3 ) + x");
    exporter::MermaidExporter::export(&parsed.tree, "mermaid");

    parsed.set_variable("x", 10.0f64).unwrap();
    parsed.set_variable("x", 1.1653414f64).unwrap();

    println!("{}", parsed.eval().unwrap());

    println!(
        "{}",
        // TODO: Macro for this (constucting a HashMap)
        parsed
            .eval_for(HashMap::from([
                (String::from("x"), 1.1653414f64),
                (String::from("z"), 0.0f64),
                (String::from("a"), 0.0f64)
            ]))
            .unwrap()
    );
}
