use itertools::Itertools;

fn part1(l1: u32, h1: u32, l2: u32, h2: u32) -> bool {
    (l1 <= l2 && h1 >= h2) || (l2 <= l1 && h2 >= h1)
}

fn part2(l1: u32, h1: u32, l2: u32, h2: u32) -> bool {
    (l1 <= l2 && h1 >= l2) || (l2 <= l1 && h2 >= l1)
}

fn run(input: &str, f: fn(u32, u32, u32, u32) -> bool) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .flat_map(|x| x.split("-"))
                .map(|x| x.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32, u32, u32)>()
                .unwrap()
        })
        .map(|(l1, h1, l2, h2)| f(l1, h1, l2, h2) as u32)
        .sum()
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(run(input, part1))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(run(input, part2))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
