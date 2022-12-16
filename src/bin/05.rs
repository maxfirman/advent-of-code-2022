use itertools::Itertools;

fn part1(crates: &mut Vec<Vec<char>>, n: usize, from: usize, to: usize) {
    for _ in 0..n {
        let val = crates[from].pop().unwrap();
        crates[to].push(val);
    }
}

fn part2(crates: &mut Vec<Vec<char>>, n: usize, from: usize, to: usize) {
    let mut tmp: Vec<char> = vec![];
    for _ in 0..n {
        let val = crates[from].pop().unwrap();
        tmp.push(val);
    }
    for _ in 0..n {
        let val = tmp.pop().unwrap();
        crates[to].push(val)
    }
}

fn run(input: &str, f: fn(&mut Vec<Vec<char>>, usize, usize, usize)) -> String {
    let (crates_input, instructions_input) = input.split_once("\n\n").unwrap();

    let mut crates_input = crates_input.lines().rev();

    let n = (crates_input.next().unwrap().len() + 2) / 4;
    let mut crates: Vec<Vec<char>> = vec![vec![]; n];

    for line in crates_input {
        for (i, c) in line
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| c.is_alphabetic())
        {
            crates[i].push(c);
        }
    }

    for line in instructions_input.lines() {
        let (n, from, to) = line
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect_tuple()
            .unwrap();
        f(&mut crates, n, from - 1, to - 1);
    }

    crates
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>()
}
pub fn part_one(input: &str) -> Option<String> {
    Some(run(input, part1))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(run(input, part2))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
