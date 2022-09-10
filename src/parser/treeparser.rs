use super::{Matpars, Operator, Parser, Type};
use std::collections::HashMap;

// Should be the maximum the values in the weights map + 1 to outpower everything outside the brackets
const BRACKETS_EXTRA_WEIGHT: u32 = 4;

pub struct TreeParser;

impl Parser for TreeParser {
    fn parse(input: &str) -> Matpars {
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

        let tree = construct_tree(&input_copy, weights.as_slice());
        tree
    }
}

fn construct_tree(input: &String, weights: &[u32]) -> Matpars {
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
            return Matpars::new(
                Type::Variable(input.to_string()),
                Option::None,
                Option::None,
            );
        };
        if input.chars().all(char::is_numeric) {
            return Matpars::new(
                Type::Constant(input.parse::<f64>().unwrap()),
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

    Matpars::new(
        Type::Operator(
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
