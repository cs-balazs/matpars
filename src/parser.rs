use std::{cell::Cell, collections::HashMap};

// Should be the maximum the values in the weights map + 1 to outpower everything outside the brackets

const BRACKETS_EXTRA_WEIGHT: u32 = 4;

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
pub enum Node {
    Operator(Operator),
    Constant(f64),
    Variable(String),
}

#[derive(Debug)]
pub struct Formula {
    pub id: usize,
    pub node: Node,
    pub left: Box<Option<Formula>>,
    pub right: Box<Option<Formula>>,
}

impl Formula {
    fn new(node: Node, left: Option<Formula>, right: Option<Formula>) -> Formula {
        NODE_ID.with(|thread_id| {
            let id = thread_id.get();
            thread_id.set(id + 1);
            Formula {
                id,
                node,
                left: Box::new(left),
                right: Box::new(right),
            }
        })
    }
}

fn construct_tree(input: &String, weights: &[u32]) -> Formula {
    let min_weight = {
        let mut minimum = u32::MAX;
        for &weight in weights {
            if weight > 0 && weight < minimum {
                minimum = weight;
            };
        }
        if minimum == u32::MAX {
            minimum = 0;
        }
        minimum
    };

    if min_weight == 0 {
        if input.chars().all(char::is_alphabetic) {
            return Formula::new(
                Node::Variable(input.to_string()),
                Option::None,
                Option::None,
            );
        };
        if input.chars().all(char::is_numeric) {
            return Formula::new(
                Node::Constant(input.parse::<f64>().unwrap()),
                Option::None,
                Option::None,
            );
        };
    };

    let rightmost_min_weight_index =
        weights.iter().enumerate().fold(
            0,
            |acc, curr| if *curr.1 == min_weight { curr.0 } else { acc },
        );
    let left = input[0..rightmost_min_weight_index].to_string();
    let right = input[rightmost_min_weight_index + 1..].to_string();
    let left_weights = &weights[0..rightmost_min_weight_index];
    let right_weights = &weights[rightmost_min_weight_index + 1..];

    Formula::new(
        Node::Operator(
            match input.chars().nth(rightmost_min_weight_index).unwrap() {
                '+' => Operator::Plus,
                '-' => Operator::Minus,
                '*' => Operator::Times,
                '/' => Operator::Division,
                '^' => Operator::Power,
                _ => panic!(),
            },
        ),
        Some(construct_tree(&left, left_weights)),
        Some(construct_tree(&right, right_weights)),
    )
}

pub fn parse(input: &String) -> Formula {
    // TODO: Somehow construct this as a const
    let weights: HashMap<char, u32> =
        HashMap::from([('+', 1), ('-', 1), ('*', 2), ('/', 2), ('^', 3)]);

    let mut input_copy = input.trim().replace(' ', "");
    let mut extra_weight: u32 = 0;
    let mut weights: Vec<u32> = input_copy
        .clone()
        .chars()
        .map(|chr| {
            if let Some(val) = weights.get(&chr) {
                *val + extra_weight
            } else {
                if chr == '(' {
                    extra_weight += BRACKETS_EXTRA_WEIGHT
                } else if chr == ')' {
                    extra_weight -= BRACKETS_EXTRA_WEIGHT
                };
                0
            }
        })
        .collect::<Vec<u32>>();

    while let Some(index) = input_copy.find(|c: char| (c == '(') || (c == ')')) {
        input_copy.remove(index);
        weights.remove(index);
    }

    // println!("{:?}", input_copy.chars().collect::<Vec<char>>());
    // println!(
    //     "{:?}",
    //     weights
    //         .iter()
    //         .map(|val| val.to_string().chars().next().unwrap())
    //         .collect::<Vec<char>>()
    // );

    let tree = construct_tree(&input_copy, weights.as_slice());
    tree
}
