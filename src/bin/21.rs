use std::collections::HashMap;
use MathOperation::*;
use Val::*;

#[derive(Debug)]
enum MathOperation {
    Add,
    Subtract,
    Divide,
    Multiply,
}

#[derive(Debug)]
enum Val {
    Operation {
        left: String,
        right: String,
        operation: MathOperation,
    },
    Number(i64),
}

fn dfs(node: &str, graph: &HashMap<String, Val>) -> i64 {
    let val = graph.get(node).unwrap();
    match val {
        Operation {
            left,
            right,
            operation,
        } => match operation {
            Add => dfs(left, graph) + dfs(right, graph),
            Subtract => dfs(left, graph) - dfs(right, graph),
            Divide => dfs(left, graph) / dfs(right, graph),
            Multiply => dfs(left, graph) * dfs(right, graph),
        },
        Number(number) => *number,
    }
}

fn create_graph(input: &str) -> HashMap<String, Val> {
    let mut graph: HashMap<String, Val> = HashMap::new();

    for line in input.lines() {
        let (key, val_str) = line.split_once(": ").unwrap();

        let val = if let Some(number) = val_str.parse::<i64>().ok() {
            Number(number)
        } else {
            Operation {
                left: String::from(&val_str[0..4]),
                right: String::from(&val_str[7..11]),
                operation: match &val_str.chars().nth(5).unwrap() {
                    '+' => Add,
                    '-' => Subtract,
                    '*' => Multiply,
                    '/' => Divide,
                    _ => panic!("Unsupported operation"),
                },
            }
        };
        graph.insert(String::from(key), val);
    }
    graph
}

pub fn part_one(input: &str) -> Option<i64> {
    let graph = create_graph(input);

    Some(dfs("root", &graph))
}
pub fn part_two(input: &str) -> Option<i64> {
    let mut graph = create_graph(input);
    let mut n = 3305669201317_i64;
    loop {
        graph.insert("humn".to_string(), Number(n));
        if match graph.get("root").unwrap() {
            Operation { left, right, .. } => {
                let l = dfs(left, &graph);
                let r = dfs(right, &graph);
                let shit = l - r;
                println!("{n} {shit}");
                l == r
            }
            Number(_) => {
                panic!("Expected root to be an Operation")
            }
        } {
            return Some(n);
        }
        n += 1;
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
