use matpars::{
    self,
    exporter::{self, FileExporter, StringExporter},
};
use std::io::{stdout, Write};

fn main() {
    let mut parsed =
        matpars::parse("3 * 1 - 12 ^ 2 + 3 * 12 + ( 8 * 42 - 21 * 9 ) + 5 / ( 10 * 2 - 3 ) + 243");

    println!("{}", exporter::PolishExporter::to_string(&parsed.tree));

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
