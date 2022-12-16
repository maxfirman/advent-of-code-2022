use itertools::Itertools;
use std::collections::HashSet;

fn score(c: &char) -> u32 {
    if c.is_ascii_lowercase() {
        *c as u32 - 96
    } else {
        *c as u32 - 38
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (left, right) = line.split_at(line.len() / 2);
                let left = left.chars().collect::<HashSet<_>>();
                let right = right.chars().collect::<HashSet<_>>();
                left.intersection(&right).map(score).sum::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| line.chars().collect::<HashSet<_>>())
            .enumerate()
            .group_by(|(i, _)| i / 3)
            .into_iter()
            .map(|(_, group)| {
                group
                    .map(|(_, chars)| chars)
                    .reduce(|left, right| left.intersection(&right).map(|c| *c).collect())
                    .unwrap()
                    .iter()
                    .map(score)
                    .sum::<u32>()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
