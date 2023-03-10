use std::cmp::max;
use std::collections::{HashMap, HashSet};

const START_POSITION: &str = "AA";
const PART1_MINUTES: usize = 30;
const PART2_MINUTES: usize = 26;

struct Caves {
    graph: Vec<Vec<usize>>,
    flow_rates: Vec<usize>,
    map: HashMap<String, usize>,
}

impl Caves {
    fn from_input(input: &str) -> Self {
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
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let map = input
            .lines()
            .map(|line| &line[6..8])
            .enumerate()
            .map(|(a, b)| (b.to_owned(), a))
            .collect::<HashMap<_, _>>();

        let graph = input
            .lines()
            .map(|line| {
                line.split_once(";")
                    .unwrap()
                    .1
                    .split_at(23)
                    .1
                    .split(", ")
                    .map(|k| *map.get(k.trim()).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let n = graph.len();
        let graph = (0..n)
            .map(|source| dijkstra(&graph, source))
            .collect::<Vec<Vec<_>>>();

        Self {
            graph,
            flow_rates,
            map,
        }
    }

    fn part1(&self) -> usize {
        self._part1(
            vec![false; self.graph.len()],
            *self.map.get(START_POSITION).unwrap(),
            PART1_MINUTES,
        )
    }

    fn part2(&self) -> usize {
        self._part2(
            vec![false; self.graph.len()],
            *self.map.get(START_POSITION).unwrap(),
            PART2_MINUTES,
        )
    }

    fn _part1(&self, mut visited: Vec<bool>, i: usize, mut minutes_remaining: usize) -> usize {
        if minutes_remaining <= 1 {
            return 0;
        }
        let mut val = 0usize;
        if !visited[i] && self.flow_rates[i] > 0 {
            minutes_remaining -= 1;
            val += self.flow_rates[i] * minutes_remaining;
            visited[i] = true;
        }
        val + self.graph[i]
            .iter()
            .enumerate()
            .filter(|(j, &d)| !visited[*j] && self.flow_rates[*j] > 0 && minutes_remaining >= d)
            .map(|(j, &d)| self._part1(visited.clone(), j, minutes_remaining - d))
            .max()
            .unwrap_or(0)
    }

    fn _part2(&self, mut visited: Vec<bool>, i: usize, mut minutes_remaining: usize) -> usize {
        if minutes_remaining <= 1 {
            return 0;
        }
        let mut val = 0usize;
        if !visited[i] && self.flow_rates[i] > 0 {
            minutes_remaining -= 1;
            val += self.flow_rates[i] * minutes_remaining;
            visited[i] = true;
        }
        val + max(
            self.graph[i]
                .iter()
                .enumerate()
                .filter(|(j, &d)| !visited[*j] && self.flow_rates[*j] > 0 && minutes_remaining >= d)
                .map(|(j, &d)| self._part2(visited.clone(), j, minutes_remaining - d))
                .max()
                .unwrap_or(0),
            self._part1(visited.clone(), 0, PART2_MINUTES),
        )
    }
}

fn dijkstra(graph: &Vec<Vec<usize>>, source: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![usize::MAX; n];
    dist[source] = 0;
    let mut q = (0..n).collect::<HashSet<_>>();
    while !q.is_empty() {
        let (u, dist_u) = q
            .iter()
            .map(|&i| (i, dist[i]))
            .min_by_key(|(_, d)| *d)
            .unwrap();
        q.remove(&u);
        for &v in graph[u].iter().filter(|&v| q.contains(v)) {
            let alt = dist_u + 1;
            if alt < dist[v] {
                dist[v] = alt;
            }
        }
    }
    dist
}


pub fn part_one(input: &str) -> Option<usize> {
    Some(Caves::from_input(input).part1())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Caves::from_input(input).part2())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
