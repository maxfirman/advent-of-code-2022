extern crate core;

use crate::Operation::{Add, Multiply};
use crate::Value::{Number, Old};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug)]
enum Value {
    Old,
    Number(u128),
}

#[derive(Debug)]
enum Operation {
    Add(Value),
    Multiply(Value),
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut items = input
        .lines()
        .skip(1)
        .step_by(7)
        .map(|line| {
            line.split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<VecDeque<_>>()
        })
        .collect::<Vec<_>>();

    let operations = input
        .lines()
        .skip(2)
        .step_by(7)
        .map(|line| {
            let operation = line.chars().nth(23).unwrap();
            let value_str = line.chars().skip(25).collect::<String>();
            let value = match value_str.as_str() {
                "old" => Old,
                _ => Number(value_str.parse::<u128>().unwrap()),
            };
            match operation {
                '+' => Add(value),
                '*' => Multiply(value),
                _ => panic!("Unsupported operation: {operation}"),
            }
        })
        .collect::<Vec<_>>();

    let divisors = input
        .lines()
        .skip(3)
        .step_by(7)
        .map(|line| {
            line.chars()
                .skip(21)
                .collect::<String>()
                .parse::<u128>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let tests = input
        .lines()
        .skip(4)
        .step_by(7)
        .zip(input.lines().skip(5).step_by(7))
        .map(|(t, f)| {
            (
                t.strip_prefix("    If true: throw to monkey ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                f.strip_prefix("    If false: throw to monkey ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();
    // println!("{tests:#?}");

    let n = items.len();
    let mut inspect_count = vec![0u128; n];

    for _ in 0..10000 {
        for i in 0..n {
            while let Some(val) = items[i].pop_front() {
                let worry_level = match &operations[i] {
                    Add(num) => match num {
                        Old => val + val,
                        Number(number) => val + number,
                    },
                    Multiply(num) => match num {
                        Old => val * val,
                        Number(number) => val * number,
                    },
                };
                let is_true = worry_level % divisors[i] == 0;
                let j = if is_true { tests[i].0 } else { tests[i].1 };
                items[j].push_back(worry_level);
                inspect_count[i] += 1;
            }
        }
    }
    Some(inspect_count.iter().sorted().rev().take(2).product())
}

pub fn part_two(input: &str) -> Option<u128> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
