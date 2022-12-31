use std::cmp::max;
use std::iter::Sum;
use std::ops;

use itertools::Itertools;
use regex::Regex;

use Robot::*;

#[derive(Hash, PartialEq, Eq, Debug, Ord, PartialOrd, Clone)]
enum Robot {
    Null,
    Ore { ore: u32 },
    Clay { ore: u32 },
    Obsidian { ore: u32, clay: u32 },
    Geode { ore: u32, obsidian: u32 },
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug)]
struct Account {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Account {
    fn can_afford(&self, robot: &Robot) -> bool {
        match robot {
            Null => true,
            Ore { ore } => self.ore >= *ore,
            Clay { ore } => self.ore >= *ore,
            Obsidian { ore, clay } => self.ore >= *ore && self.clay >= *clay,
            Geode { ore, obsidian } => self.ore >= *ore && self.obsidian >= *obsidian,
        }
    }
}

impl Account {
    fn new() -> Account {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
    fn ore() -> Account {
        Self {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

impl Sum for Account {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or(Account::new())
    }
}

impl ops::Add<Account> for Account {
    type Output = Self;

    fn add(self, rhs: Account) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl ops::AddAssign<Account> for Account {
    fn add_assign(&mut self, rhs: Account) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl ops::Sub<Robot> for Account {
    type Output = Self;

    fn sub(self, rhs: Robot) -> Self::Output {
        match rhs {
            Null => self,
            Ore { ore } => Self {
                ore: self.ore - ore,
                clay: self.clay,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Clay { ore } => Self {
                ore: self.ore - ore,
                clay: self.clay,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Obsidian { ore, clay } => Self {
                ore: self.ore - ore,
                clay: self.clay - clay,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Geode { ore, obsidian } => Self {
                ore: self.ore - ore,
                clay: self.clay,
                obsidian: self.obsidian - obsidian,
                geode: self.geode,
            },
        }
    }
}

struct Game {
    running_max: u32,
}

impl Game {
    fn new() -> Self {
        Self { running_max: 0u32 }
    }
    fn dfs(
        &mut self,
        mut account: Account,
        mut robots: Account,
        robot_to_build: Robot,
        remaining: u32,
        cost: &[Robot; 5],
    ) -> u32 {
        if account.geode + robots.geode * remaining + (remaining - 1) * remaining / 2
            < self.running_max
        {
            return 0;
        }
        // Base Condition
        if remaining == 1 {
            self.running_max = max(self.running_max, account.geode + robots.geode);
            return self.running_max;
        }

        // Accrue
        account += robots;

        // Build
        match robot_to_build {
            Ore { .. } => robots.ore += 1,
            Clay { .. } => robots.clay += 1,
            Obsidian { .. } => robots.obsidian += 1,
            Geode { .. } => robots.geode += 1,
            Null => {}
        }

        let result = cost
            .iter()
            .filter(|x| match x {
                Ore { .. } => {
                    robots.ore
                        < cost
                            .iter()
                            .map(|x| match x {
                                Null => 0u32,
                                Ore { ore } => *ore,
                                Clay { ore } => *ore,
                                Obsidian { ore, .. } => *ore,
                                Geode { ore, .. } => *ore,
                            })
                            .max()
                            .unwrap()
                        && account.can_afford(*x)
                }
                Clay { .. } => {
                    robots.clay
                        < cost
                            .iter()
                            .map(|x| match x {
                                Obsidian { clay, .. } => *clay,
                                _ => 0u32,
                            })
                            .max()
                            .unwrap()
                        && account.can_afford(*x)
                }
                Obsidian { .. } => {
                    robots.obsidian
                        < cost
                            .iter()
                            .map(|x| match x {
                                Geode { obsidian, .. } => *obsidian,
                                _ => 0u32,
                            })
                            .max()
                            .unwrap()
                        && account.can_afford(*x)
                }
                Geode { .. } => account.can_afford(*x),
                Null => !account.can_afford(&cost[4]),
            })
            .map(|robot_to_build| {
                self.dfs(
                    account - robot_to_build.clone(),
                    robots,
                    robot_to_build.clone(),
                    remaining - 1,
                    cost,
                )
            })
            .max()
            .unwrap();
        result
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    Some(
        input
            .lines()
            .map(|line| {
                re.captures_iter(line)
                    .filter_map(|cap| {
                        cap.iter()
                            .skip(1)
                            .map(|x| x.unwrap().as_str().parse::<u32>().unwrap())
                            .collect_tuple::<(u32, u32, u32, u32, u32, u32, u32)>()
                    })
                    .collect_tuple::<(_,)>()
                    .unwrap()
            })
            .map(|(x,)| {
                [
                    Null,
                    Ore { ore: x.1 },
                    Clay { ore: x.2 },
                    Obsidian {
                        ore: x.3,
                        clay: x.4,
                    },
                    Geode {
                        ore: x.5,
                        obsidian: x.6,
                    },
                ]
            })
            .map(|cost| {
                let mut game = Game::new();
                game.dfs(Account::new(), Account::ore(), Null, 24, &cost)
            })
            .enumerate()
            .map(|(i, x)| (i as u32 + 1) * x)
            .sum::<u32>(),
    )
}

// pub fn part_two(input: &str) -> Option<u32> {
//     None
// }

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    // advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 19);
    //     assert_eq!(part_two(&input), None);
    // }
}
