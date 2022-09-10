use crate::parser::Matpars;

mod mermaid;

pub use mermaid::MermaidExporter;

pub trait FileExporter {
    fn export(parsed: &Matpars, filename: &str);
}
