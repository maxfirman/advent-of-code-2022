use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<i32> {
    let cubes = get_cubes(input);

    Some(
        cubes
            .iter()
            .map(|(x, y, z)| {
                6 - [
                    (x - 1, *y, *z),
                    (x + 1, *y, *z),
                    (*x, *y - 1, *z),
                    (*x, *y + 1, *z),
                    (*x, *y, z - 1),
                    (*x, *y, z + 1),
                ]
                .iter()
                .map(|x| cubes.contains(x) as i32)
                .sum::<i32>()
            })
            .sum(),
    )
}

fn get_cubes(input: &str) -> HashSet<(i8, i8, i8)> {
    let cubes = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i8>().unwrap())
                .collect_tuple::<(i8, i8, i8)>()
                .unwrap()
        })
        .collect::<HashSet<_>>();
    cubes
}

pub fn part_two(input: &str) -> Option<u32> {
    let cubes = get_cubes(input);

    let i_min = *cubes.iter().map(|(i, _, _)| i).min().unwrap() - 1;
    let i_max = *cubes.iter().map(|(i, _, _)| i).max().unwrap() + 1;
    let j_min = *cubes.iter().map(|(_, j, _)| j).min().unwrap() - 1;
    let j_max = *cubes.iter().map(|(_, j, _)| j).max().unwrap() + 1;
    let k_min = *cubes.iter().map(|(_, _, k)| k).min().unwrap() - 1;
    let k_max = *cubes.iter().map(|(_, _, k)| k).max().unwrap() + 1;

    let mut queue: VecDeque<(i8, i8, i8)> = VecDeque::new();
    let mut sides = 0u32;
    let mut visited: HashSet<(i8, i8, i8)> = HashSet::new();

    queue.push_back((i_min, j_min, k_min));
    while !queue.is_empty() {
        let (i, j, k) = queue.pop_front().unwrap();
        for point in [
            (i - 1, j, k),
            (i + 1, j, k),
            (i, j - 1, k),
            (i, j + 1, k),
            (i, j, k - 1),
            (i, j, k + 1),
        ]
        .iter()
        .filter(|(ii, jj, kk)| {
            i_min <= *ii
                && *ii <= i_max
                && j_min <= *jj
                && *jj <= j_max
                && k_min <= *kk
                && *kk <= k_max
        }) {
            if cubes.contains(point) {
                sides += 1;
            } else if !visited.contains(point) {
                queue.push_back(*point);
                visited.insert(*point);
            }
        }
    }
    Some(sides)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
