use std::cmp::max;

fn get_wood(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let wood = get_wood(input);
    let m = wood.len();
    let n = wood[0].len();
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            res += (i == 0
                || j == 0
                || i == m - 1
                || j == n - 1
                || wood[i][j] > (0..i).map(|ii| wood[ii][j]).max().unwrap()
                || wood[i][j] > (i + 1..m).map(|ii| wood[ii][j]).max().unwrap()
                || wood[i][j] > (0..j).map(|jj| wood[i][jj]).max().unwrap()
                || wood[i][j] > (j + 1..n).map(|jj| wood[i][jj]).max().unwrap())
                as u32;
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let wood = get_wood(input);

    let m = wood.len();
    let n = wood[0].len();
    let mut res: u32 = 0;

    for i in 0..m {
        for j in 0..n {
            let mut a = 0;
            for ii in (0..i).rev() {
                a += 1;
                if wood[ii][j] >= wood[i][j] {
                    break;
                }
            }
            let mut b = 0;
            for ii in i + 1..m {
                b += 1;
                if wood[ii][j] >= wood[i][j] {
                    break;
                }
            }
            let mut c = 0;
            for jj in (0..j).rev() {
                c += 1;
                if wood[i][jj] >= wood[i][j] {
                    break;
                }
            }
            let mut d = 0;
            for jj in j + 1..n {
                d += 1;
                if wood[i][jj] >= wood[i][j] {
                    break;
                }
            }
            res = max(res, a * b * c * d)
        }
    }
    Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
