use std::io::{stdout, Write};

use matpars::{
    self,
    exporter::{self, FileExporter},
};

fn main() {
    let mut parsed =
        matpars::parse("3 * z - x ^ 2 + 3 * x + ( 8 * x - 21 * 9 ) + 5 / ( 10 * a - 3 ) + x");

    for name in parsed.variables.clone().keys() {
        print!("{} = ", name);
        stdout().flush().unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        parsed
            .variables
            .insert(name.to_string(), line.trim().parse::<f64>().unwrap());
    }

    println!("y = {}", parsed.eval().unwrap());

    exporter::MermaidExporter::export(&parsed.tree, "mermaid");
}
