use crate::util::input::read_buf_ints;
use crate::util::Part;

crate::create_solver!(part_one, part_two);

fn part_one(input: Vec<u8>) -> i64 {
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

    result
}

fn part_two(input: Vec<u8>) -> i64 {
    let nums = read_buf_ints(&input);

    let (left, right) = nums
        .chunks(2)
        .map(|pair| (pair[0], pair[1]))
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let result = left
        .iter()
        .map(|l| right.iter().filter(|&r| l == r).count() as i64 * l)
        .sum::<i64>();

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
    fn test_part_one() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part_one(input), 11);
    }

    #[test]
    fn test_part_two() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part_two(input), 31);
    }
}
