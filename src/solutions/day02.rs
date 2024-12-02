use crate::util::{
    input::{read_lines, read_str_ints},
    Part,
};

crate::create_solver!(part_one, part_two);

fn is_safe(a: &i64, b: &i64, asc: bool) -> bool {
    let diff = b - a;
    if asc {
        diff >= 1 && diff <= 3
    } else {
        diff >= -3 && diff <= -1
    }
}

fn is_safe_report(report: &Vec<i64>) -> bool {
    let asc = report.first().unwrap() < report.last().unwrap();
    report
        .iter()
        .zip(report.iter().skip(1))
        .all(|(a, b)| is_safe(a, b, asc))
}

fn part_one(input: Vec<u8>) -> i64 {
    let reports = read_lines(&input)
        .iter()
        .map(|line| read_str_ints(line))
        .collect::<Vec<Vec<i64>>>();

    let safe = reports
        .iter()
        .map(|report| is_safe_report(report))
        .filter(|&safe| safe)
        .count() as i64;

    safe
}

fn part_two(input: Vec<u8>) -> i64 {
    let reports = read_lines(&input)
        .iter()
        .map(|line| read_str_ints(line))
        .collect::<Vec<Vec<i64>>>();

    let safe = reports
        .iter()
        .map(|report| {
            report
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    report
                        .iter()
                        .enumerate()
                        .filter_map(|(j, &v)| if i != j { Some(v) } else { None })
                        .collect::<Vec<i64>>()
                })
                .any(|report| is_safe_report(&report))
        })
        .filter(|&safe| safe)
        .count() as i64;

    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_part_one() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn test_part_two() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part_two(input), 4);
    }
}
