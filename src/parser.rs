use std::cell::Cell;

mod treeparser;

pub use treeparser::TreeParser;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Division,
    Power,
}

thread_local!(static NODE_ID: Cell<usize> = Cell::new(0));

#[derive(Debug)]
pub enum Type {
    Operator(Operator),
    Constant(f64),
    Variable(String),
}

#[derive(Debug)]
pub struct Matpars {
    pub id: usize,
    pub node_type: Type,
    pub left: Box<Option<Matpars>>,
    pub right: Box<Option<Matpars>>,
}

impl Matpars {
    fn new(node_type: Type, left: Option<Matpars>, right: Option<Matpars>) -> Matpars {
        NODE_ID.with(|thread_id| {
            let id = thread_id.get();
            thread_id.set(id + 1);
            Matpars {
                id,
                node_type,
                left: Box::new(left),
                right: Box::new(right),
            }
        })
    }
}

pub trait Parser {
    fn parse(input: &str) -> Matpars;
}
