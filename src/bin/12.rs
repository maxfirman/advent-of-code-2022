extern crate core;

use std::collections::{HashMap, HashSet};

fn dijkstra(
    graph: HashMap<(usize, usize), Vec<(usize, usize)>>,
    source: (usize, usize),
    target: (usize, usize),
) -> usize {
    let mut dist = graph
        .iter()
        .map(|(k, _)| (*k, usize::MAX))
        .collect::<HashMap<(usize, usize), usize>>();
    dist.insert(source, 0);

    let mut q = graph.iter().map(|(k, _)| *k).collect::<HashSet<_>>();
    while !q.is_empty() {
        let (u, dist_u) = q
            .iter()
            .map(|&k| (k, *dist.get(&k).unwrap()))
            .min_by_key(|(_, d)| *d)
            .unwrap();
        if u == target {
            return dist_u;
        }
        q.remove(&u);
        for &v in graph.get(&u).unwrap().iter().filter(|&v| q.contains(v)) {
            let alt = dist_u + 1;
            if alt < *dist.get(&v).unwrap() {
                dist.insert(v, alt);
            }
        }
    }
    panic!(" Did not find target node!")
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => 0u8,
                    'E' => 27u8,
                    _ => c as u8 - 96u8,
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<_>>>();

    let m = map.len();
    let n = map[0].len();

    let mut is = 0usize;
    let mut js = 0usize;
    let mut it = 0usize;
    let mut jt = 0usize;

    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

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
                .filter(|&(ii, jj)| map[ii][jj] <= map[i][j] + 1)
                .collect::<Vec<(usize, usize)>>(),
            );
            if map[i][j] == 0 {
                is = i;
                js = j;
            }
            if map[i][j] == 27 {
                it = i;
                jt = j;
            }
        }
    }

    Some(dijkstra(graph, (is, js), (it, jt)))
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
