use std::collections::{HashMap, HashSet};

struct Caves {
    graph: Vec<Vec<usize>>,
    flow_rates: Vec<usize>,
}

impl Caves {
    fn new(graph: Vec<Vec<usize>>, flow_rates: Vec<usize>) -> Self {
        Self { graph, flow_rates }
    }

    fn dfs(&self, mut visited: [bool; 60], i: usize, mut minutes_remaining: usize) -> usize {
        if minutes_remaining <= 0 {
            return 0;
        }
        let mut val = 0usize;
        if !visited[i] && self.flow_rates[i] > 0 {
            minutes_remaining -= 1;
            val += self.flow_rates[i] * minutes_remaining;
        }
        visited[i] = true;
        val + self.graph[i]
            .iter()
            .enumerate()
            .filter(|(j, &d)| !visited[*j] && self.flow_rates[*j] > 0 && minutes_remaining >= d)
            .map(|(j, &d)| self.dfs(visited, j, minutes_remaining - d))
            .max()
            .unwrap_or(0)
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
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();

    println!("{map:?}");
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

    println!("{graph:?}");
    let dist = (0..graph.len())
        .map(|source| dijkstra(&graph, source))
        .collect::<Vec<Vec<_>>>();
    println!("{:?}", dist);

    let visited = [false; 60];
    let caves = Caves::new(dist, flow_rates);
    Some(caves.dfs(visited, 0, 30))
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
