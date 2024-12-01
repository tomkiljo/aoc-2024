use crate::util::input::{read_buf_ints, read_file};

pub fn run(part: &str) {
    let input = read_file("input/day01.txt");
    match part {
        "1" => part1(input),
        "2" => part2(input),
        _ => panic!("Invalid part {}", part),
    };
}

fn part1(input: Vec<u8>) -> i64 {
    let nums = read_buf_ints(&input);

    let (mut left, mut right) = nums
        .chunks(2)
        .map(|pair| (pair[0], pair[1]))
        .unzip::<_, _, Vec<_>, Vec<_>>();

    left.sort();
    right.sort();

    let result = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i64>();

    println!("Day 1, Part 1: {}", result);
    result
}

fn part2(input: Vec<u8>) -> i64 {
    let nums = read_buf_ints(&input);

    let (left, right) = nums
        .chunks(2)
        .map(|pair| (pair[0], pair[1]))
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let result = left
        .iter()
        .map(|l| right.iter().filter(|&r| l == r).count() as i64 * l)
        .sum::<i64>();

    println!("Day 1, Part 2: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test_part1() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part2(input), 31);
    }
}
