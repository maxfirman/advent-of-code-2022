extern crate core;

use std::collections::{HashMap, HashSet};

fn dijkstra(
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
) -> HashMap<(usize, usize), usize> {
    let mut dist = graph
        .iter()
        .map(|(k, _)| (*k, 1_000_000))
        .collect::<HashMap<(usize, usize), usize>>();
    dist.insert(start, 0);

    let mut q = graph.iter().map(|(k, _)| *k).collect::<HashSet<_>>();
    while !q.is_empty() {
        let (u, dist_u) = q
            .iter()
            .map(|&k| (k, *dist.get(&k).unwrap()))
            .min_by_key(|(_, d)| *d)
            .unwrap();
        q.remove(&u);
        for &v in graph.get(&u).unwrap().iter().filter(|&v| q.contains(v)) {
            let alt = dist_u + 1;
            if alt < *dist.get(&v).unwrap() {
                dist.insert(v, alt);
            }
        }
    }
    dist
}

pub fn run(
    input: &str,
) -> (
    HashMap<(usize, usize), usize>,
    HashMap<usize, Vec<(usize, usize)>>,
) {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => 0usize,
                    'E' => 27usize,
                    _ => c as usize - 96usize,
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<_>>>();

    let m = map.len();
    let n = map[0].len();

    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut inverse_height_map: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..m {
        for j in 0..n {
            graph.insert(
                (i, j),
                [
                    (i as i32 - 1, j as i32),
                    (i as i32 + 1, j as i32),
                    (i as i32, j as i32 + 1),
                    (i as i32, j as i32 - 1),
                ]
                .iter()
                .filter(|&&(ii, jj)| {
                    ii >= 0 && jj >= 0 && ii < map.len() as i32 && jj < map[0].len() as i32
                })
                .map(|&(ii, jj)| (ii as usize, jj as usize))
                .filter(|&(ii, jj)| map[ii][jj] + 1 >= map[i][j])
                .collect::<Vec<(usize, usize)>>(),
            );
            inverse_height_map
                .entry(map[i][j])
                .or_default()
                .push((i, j));
        }
    }
    let target = inverse_height_map.get(&27usize).unwrap()[0];
    let dist = dijkstra(&graph, target);
    (dist, inverse_height_map)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (dist, inverse_height_map) = run(input);
    let source = inverse_height_map.get(&0usize).unwrap()[0];
    Some(*dist.get(&source).unwrap())
}
pub fn part_two(input: &str) -> Option<usize> {
    let (dist, inverse_height_map) = run(input);
    inverse_height_map
        .get(&1usize)
        .unwrap()
        .iter()
        .map(|&source| *dist.get(&source).unwrap())
        .min()
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
        assert_eq!(part_two(&input), Some(29));
    }
}
