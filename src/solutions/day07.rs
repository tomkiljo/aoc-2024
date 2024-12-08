use crate::{
    create_solver,
    util::input::{read_lines, read_str_ints},
    Part,
};

create_solver!(part_one, part_two);

fn part_one(input: Vec<u8>) -> i64 {
    fn can_solve(result: i64, value: i64, numbers: &[i64]) -> bool {
        if let Some(n) = numbers.first() {
            let add = value + n;
            if add <= result && can_solve(result, add, &numbers[1..]) {
                return true;
            }
            let mul = value * n;
            if mul <= result && can_solve(result, mul, &numbers[1..]) {
                return true;
            }
        } else {
            return value == result;
        }
        false
    }

    read_lines(&input)
        .iter()
        .map(|line| read_str_ints(&line))
        .map(|ints| {
            let result = ints[0];
            let value = ints[1];
            let numbers = &ints[2..];
            if can_solve(result, value, numbers) {
                result
            } else {
                0
            }
        })
        .sum()
}

fn part_two(input: Vec<u8>) -> i64 {
    fn can_solve(result: i64, value: i64, numbers: &[i64]) -> bool {
        if let Some(n) = numbers.first() {
            let add = value + n;
            if add <= result && can_solve(result, add, &numbers[1..]) {
                return true;
            }
            let mul = value * n;
            if mul <= result && can_solve(result, mul, &numbers[1..]) {
                return true;
            }
            //let con: i64 = value * 10_i64.pow(value.ilog10()) + n;
            let con: i64 = format!("{}{}", value, n).parse().unwrap();
            if con <= result && can_solve(result, con, &numbers[1..]) {
                return true;
            }
        } else {
            return value == result;
        }
        false
    }

    read_lines(&input)
        .iter()
        .map(|line| read_str_ints(&line))
        .map(|ints| {
            let result = ints[0];
            let value = ints[1];
            let numbers = &ints[2..];
            if can_solve(result, value, numbers) {
                result
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_part_one() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part_one(input.to_vec()), 3749);
    }

    #[test]
    fn test_part_two() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part_two(input.to_vec()), 11387);
    }
}
