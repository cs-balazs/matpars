use crate::parser::Tree;

mod mermaid;

pub use mermaid::MermaidExporter;

pub trait FileExporter {
    fn export(parsed: &Tree, filename: &str);
}
