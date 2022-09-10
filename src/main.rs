use matpars::{
    exporter::{self, FileExporter},
    parser,
};

fn main() {
    let parsed = parser::treeparser::parse("x^2 + 3*x + (8*x - 21*9) + 5");
    exporter::mermaid::MermaidParser::export(&parsed, "mermaid");
}
