use crate::Tree;

mod mermaid;
mod polish;

pub use mermaid::MermaidExporter;
pub use polish::PolishExporter;

pub trait FileExporter {
    fn export(parsed: &Tree, filename: &str);
}

pub trait StringExporter {
    fn to_string(parsed: &Tree) -> String;
}
