use matpars::{
    exporter::{self, FileExporter},
    parser::{self, Parser},
};

fn main() {
    let parsed = parser::TreeParser::parse("x^2 + 3*x + (8*x - 21*9) + 5");
    exporter::MermaidExporter::export(&parsed, "mermaid");
}
