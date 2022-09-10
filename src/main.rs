use matpars::{exporter::mermaid::export, parser};

fn main() {
    let parsed = parser::parse("x^2 + 3*x + (8*x - 21*9) + 5");
    export(parsed);
}
