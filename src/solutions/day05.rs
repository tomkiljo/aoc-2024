use std::collections::HashMap;

use crate::{
    create_solver,
    util::input::{read_lines, read_str_ints},
    Part,
};

create_solver!(part_one, part_two);

fn part_one(input: Vec<u8>) -> i64 {
    let lines = read_lines(&input);
    let mut rules = HashMap::<i64, Vec<i64>>::new();

    let mut result = 0;
    let mut parse_rules = true;

    for line in lines {
        if line.is_empty() {
            parse_rules = false;
            continue;
        }

        if parse_rules {
            let ints = read_str_ints(&line);
            assert!(ints.len() == 2);
            if !rules.contains_key(&ints[0]) {
                rules.insert(ints[0], Vec::new());
            }
            rules.get_mut(&ints[0]).unwrap().push(ints[1]);
        } else {
            let ints = read_str_ints(&line);
            if ints.is_sorted_by(|a, b| {
                if let Some(after) = rules.get(a) {
                    if after.contains(b) {
                        return true;
                    }
                }
                if let Some(after) = rules.get(b) {
                    if after.contains(a) {
                        return false;
                    }
                }
                true
            }) {
                result += ints[(ints.len() - 1) / 2]
            };
        }
    }

    result
}

fn part_two(input: Vec<u8>) -> i64 {
    let lines = read_lines(&input);
    let mut rules = HashMap::<i64, Vec<i64>>::new();

    let mut result = 0;
    let mut parse_rules = true;

    for line in lines {
        if line.is_empty() {
            parse_rules = false;
            continue;
        }

        if parse_rules {
            let ints = read_str_ints(&line);
            assert!(ints.len() == 2);
            if !rules.contains_key(&ints[0]) {
                rules.insert(ints[0], Vec::new());
            }
            rules.get_mut(&ints[0]).unwrap().push(ints[1]);
        } else {
            let mut ints = read_str_ints(&line);
            if !ints.is_sorted_by(|a, b| {
                if let Some(after) = rules.get(a) {
                    if after.contains(b) {
                        return true;
                    }
                }
                if let Some(after) = rules.get(b) {
                    if after.contains(a) {
                        return false;
                    }
                }
                true
            }) {
                ints.sort_by(|a, b| {
                    if let Some(after) = rules.get(a) {
                        if after.contains(b) {
                            return std::cmp::Ordering::Less;
                        }
                    }
                    if let Some(after) = rules.get(b) {
                        if after.contains(a) {
                            return std::cmp::Ordering::Greater;
                        }
                    }
                    std::cmp::Ordering::Equal
                });
                result += ints[(ints.len() - 1) / 2]
            };
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT.as_bytes().to_vec()), 143);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(INPUT.as_bytes().to_vec()), 123);
    }
}
