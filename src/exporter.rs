use crate::parser::Matpars;

pub mod mermaid;

pub trait FileExporter {
    fn export(parsed: &Matpars, filename: &str);
}
