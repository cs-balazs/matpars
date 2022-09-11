use matpars::{
    exporter::{self, FileExporter},
    parser::{self, Parser},
};

fn main() {
    let parsed =
        parser::TreeParser::parse("3* z - x^2 + 3*x + (8*x - 21*9) + 5 / ( 10 * alma - 3 ) + x");
    exporter::MermaidExporter::export(&parsed, "mermaid");
    println!("{:?}", parsed.variables);
}
