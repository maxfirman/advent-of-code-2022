fn directories(input: &str) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut dirs: Vec<i32> = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ cd ..") {
            let dir_size = stack.pop().unwrap();
            let n = stack.len();
            stack[n - 1] += dir_size;
            dirs.push(dir_size);
        } else if line.starts_with("$ cd") {
            stack.push(0);
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        } else {
            let size = line.split_once(" ").unwrap().0.parse::<i32>().unwrap();
            let n = stack.len();
            stack[n - 1] += size;
        }
    }
    loop {
        let dir_size = stack.pop().unwrap();
        dirs.push(dir_size);
        let n = stack.len();
        if n == 0 {
            break;
        }
        stack[n - 1] += dir_size;
    }
    dirs
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        directories(input)
            .into_iter()
            .filter(|x| *x <= 100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let total_disk_space = 70000000;
    let desired_free_space = 30000000;
    let mut dirs = directories(input);
    dirs.sort();
    let used_space = dirs.last().unwrap();
    let free_space = total_disk_space - used_space;
    let minimum_directory_size_to_delete = desired_free_space - free_space;
    for x in dirs {
        if x >= minimum_directory_size_to_delete {
            return Some(x);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
