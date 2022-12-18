use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    let flow_rates = input
        .lines()
        .map(|line| {
            line.split_once(";")
                .unwrap()
                .0
                .split_once("=")
                .unwrap()
                .1
                .parse::<usize>()
        })
        .collect::<Vec<_>>();
    let map = input
        .lines()
        .map(|line| &line[6..8])
        .enumerate()
        .collect::<HashMap<_, _>>();

    println!("{map:?}");
    let graph = input
        .lines()
        .map(|line| {
            line.split_once(";")
                .unwrap()
                .1
                .split_at(24)
                .1
                .split(", ")
                .map(|k| map.get(k).unwrap())
                .parse::<usize>()
        })
        .collect::<Vec<_>>();
}

// pub fn part_two(input: &str) -> Option<usize> {
//     None
// }

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    // advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 16);
    //     assert_eq!(part_two(&input), None);
    // }
}
