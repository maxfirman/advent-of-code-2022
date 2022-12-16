use std::collections::BinaryHeap;
use std::iter::Map;

fn run<'a>(input: &'a str) -> Map<std::str::Split<'a, &'a str>, fn(&'a str) -> u32> {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
}

pub fn part_one(input: &str) -> Option<u32> {
    run(input).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(run(input).collect::<BinaryHeap<_>>().iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
