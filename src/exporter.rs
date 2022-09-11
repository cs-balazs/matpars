use crate::Matpars;

mod mermaid;
mod polish;

pub use mermaid::MermaidExporter;
pub use polish::PolishExporter;

pub trait FileExporter {
    fn export(parsed: &Matpars, filename: &str);
}

pub trait StringExporter {
    fn to_string(parsed: &Matpars) -> String;
}
