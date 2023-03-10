fn run(input: &str, n: usize) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(n)
        .map(|x| (1..x.len()).any(|i| x[i..].contains(&x[i - 1])))
        .take_while(|x| *x)
        .enumerate()
        .map(|(i, _)| i + n + 1)
        .last()
        .unwrap()
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(run(input, 4))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(run(input, 14))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
