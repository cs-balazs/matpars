use std::{cell::Cell, collections::HashSet};

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
    pub variables: Vec<String>,
    pub id: usize,
    pub node_type: Type,
    pub left: Box<Option<Matpars>>,
    pub right: Box<Option<Matpars>>,
}

impl Matpars {
    fn new(node_type: Type, left: Option<Matpars>, right: Option<Matpars>) -> Matpars {
        NODE_ID.with(|thread_id| Matpars {
            variables: {
                let mut variables: HashSet<String> = HashSet::new();

                if let Type::Variable(ref var) = node_type {
                    variables.insert(var.to_string());
                }
                if let Some(ref left) = left {
                    collect_variables(left, &mut variables);
                }
                if let Some(ref right) = right {
                    collect_variables(right, &mut variables);
                }

                let mut variables_vec = variables.into_iter().collect::<Vec<String>>();
                variables_vec.sort();
                variables_vec
            },
            id: {
                let id = thread_id.get();
                thread_id.set(id + 1);
                id
            },
            node_type,
            left: Box::new(left),
            right: Box::new(right),
        })
    }
}

pub trait Parser {
    fn parse(input: &str) -> Matpars;
}

fn collect_variables(tree: &Matpars, variables: &mut HashSet<String>) {
    if let Type::Variable(var) = &tree.node_type {
        variables.insert(var.to_string());
    }

    if let Some(left) = &*tree.left {
        collect_variables(left, variables);
    }

    if let Some(right) = &*tree.right {
        collect_variables(right, variables);
    }
}
