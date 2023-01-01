fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            5_i64.pow(i as u32)
                * match c {
                    '=' => -2,
                    '-' => -1,
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    _ => panic!("Unrecognised character"),
                }
        })
        .sum()
}

fn decimal_to_snafu(decimal: i64) -> String {
    let mut n = decimal.ilog(5) as usize;
    let mut digits = vec![0; n + 1];

    let mut remaining = decimal;
    loop {
        let digit = remaining / 5_i64.pow(n as u32);
        digits[n] += digit;
        for m in n..digits.len() - 1 {
            if digits[m] > 2 {
                digits[m + 1] += 1;
                digits[m] -= 5;
            }
        }
        remaining %= 5_i64.pow(n as u32);
        if n == 0 {
            break;
        }
        n -= 1;
    }
    digits
        .iter()
        .rev()
        .map(|&x| match x {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            _ => panic!("Unexpected digit"),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let result = input.lines().map(snafu_to_decimal).sum();
    Some(decimal_to_snafu(result))
}

// pub fn part_two(input: &str) -> Option<i64> {
//     None
// }

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    // advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some(String::from("2=-1=0")));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 25);
    //     assert_eq!(part_two(&input), None);
    // }
}
