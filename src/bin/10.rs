pub fn run(input: &str) -> (i32, String) {
    let mut x = 1_i32;
    let mut i = 1_i32;
    let mut s = 0_i32;
    let mut string = String::new();
    println!("{string}");

    for line in input.lines() {
        draw_pixel(&i, &x, &mut string);
        if (i - 20) % 40 == 0 {
            s += i * x;
        }

        if line == "noop" {
            i += 1;
        } else if line.starts_with("addx") {
            i += 1;
            draw_pixel(&i, &x, &mut string);
            if (i - 20) % 40 == 0 {
                s += i * x;
            }
            x += line.split_once(" ").unwrap().1.parse::<i32>().unwrap();
            i += 1;
        }
    }
    (s, string)
}

fn draw_pixel(i: &i32, x: &i32, string: &mut String) {
    if (i - 1) % 40 == *x || (i - 1) % 40 == x - 1 || (i - 1) % 40 == x + 1 {
        string.push('#');
    } else {
        string.push('.');
    }
    if i % 40 == 0 {
        string.push('\n');
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(run(input).0)
}

pub fn part_two(input: &str) -> Option<String> {
    Some(run(input).1)
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

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(String::from(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            ))
        );
    }
}
