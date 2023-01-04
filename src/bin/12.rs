use crate::Square::*;
use cached::proc_macro::cached;
use std::collections::HashSet;

#[derive(Clone, Eq, PartialEq, Hash)]
enum Square {
    Start,
    End,
    Normal(u8),
}

#[cached]
fn dfs(i: i32, j: i32, depth: i32, map: Vec<Vec<Square>>, mut visited: Vec<(i32, i32)>) -> i32 {
    if i < 0 || i > map.len() as i32 || j < 0 || j > map[0].len() as i32 {
        return 1_000_000_000;
    }

    visited.push((i, j));

    let predicate = |(ii, jj): &&(i32, i32)| -> bool {
        *ii >= 0
            && *jj >= 0
            && *ii < map.len() as i32
            && *jj < map[0].len() as i32
            && !visited.contains(&(*ii, *jj))
    };

    match map[i as usize][j as usize] {
        Start => [(i - 1, j), (i + 1, j), (i, j + 1), (i, j - 1)]
            .iter()
            .filter(predicate)
            .map(|(ii, jj)| dfs(*ii, *jj, depth + 1, map.clone(), visited.clone()))
            .min()
            .unwrap(),
        Normal(height) => [(i - 1, j), (i + 1, j), (i, j + 1), (i, j - 1)]
            .iter()
            .filter(predicate)
            .filter(|(ii, jj)| match map[*ii as usize][*jj as usize] {
                Start => false,
                Normal(next_height) => next_height <= height + 1,
                End => height == 26,
            })
            .map(|(ii, jj)| dfs(*ii, *jj, depth + 1, map.clone(), visited.clone()))
            .min()
            .unwrap_or(1_000_000_000),
        End => depth,
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Start,
                    'E' => End,
                    _ => Normal(c as u8 - 96u8),
                })
                .collect::<Vec<Square>>()
        })
        .collect::<Vec<Vec<_>>>();

    let m = map.len();
    let n = map[0].len();

    let mut is = 0i32;
    let mut js = 0i32;

    for i in 0..m {
        for j in 0..n {
            if let Start = map[i][j] {
                is = i as i32;
                js = j as i32;
                break;
            }
        }
    }

    let visited: Vec<(i32, i32)> = Vec::new();
    Some(dfs(is, js, 0, map, visited))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
