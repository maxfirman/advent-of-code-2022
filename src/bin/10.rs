pub fn part_one(input: &str) -> Option<i32> {
    let mut x = 1_i32;
    let mut i = 1_i32;
    let mut s = 0_i32;
    for line in input.lines() {
        if (i - 20) % 40 == 0 {
            s += i * x;
        }

        if line == "noop" {
            i += 1;
        } else if line.starts_with("addx") {
            i += 1;
            if (i - 20) % 40 == 0 {
                s += i * x;
            }
            x += line.split_once(" ").unwrap().1.parse::<i32>().unwrap();
            i += 1;
        }
    }
    Some(s)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 10);
    //     assert_eq!(part_two(&input), None);
    // }
}
