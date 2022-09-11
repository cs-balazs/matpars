use matpars::{
    self,
    exporter::{self, FileExporter},
};
use std::io::{stdin, stdout, Write};

fn main() {
    print!("formula = ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut parsed = matpars::parse(input.as_str());

    for name in parsed.variables.clone().keys() {
        print!("{} = ", name);
        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        parsed
            .variables
            .insert(name.to_string(), line.trim().parse::<f64>().unwrap());
    }

    println!("y = {}", parsed.eval().unwrap());

    exporter::MermaidExporter::export(&parsed.tree, "mermaid");
}
