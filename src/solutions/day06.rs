use std::{collections::HashSet, vec};

use crate::{
    create_solver,
    util::{
        input::read_matrix,
        matrix::{Direction, MatrixOps, Point2D},
    },
    Part,
};

create_solver!(part_one, part_two);

fn part_one(input: Vec<u8>) -> i64 {
    let matrix = read_matrix(&input);

    let mut direction = Direction::North;
    let binding = matrix.iter_points().collect::<Vec<_>>();
    let mut guard = *binding
        .iter()
        .find(|(_, d)| *d == Some(&b'^'))
        .map(|(p, _)| p)
        .unwrap();
    let mut positions = vec![guard.clone()];

    loop {
        let next = guard.step(direction);
        match matrix.at_point(&next) {
            None => {
                break;
            }
            Some(b'#') => {
                direction = match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                    _ => panic!("Invalid direction"),
                };
            }
            _ => {
                guard = next;
                positions.push(guard.clone());
            }
        };
    }

    positions.sort();
    positions.dedup();
    positions.len() as i64
}

fn part_two(input: Vec<u8>) -> i64 {
    let matrix = read_matrix(&input);

    let starting_point = matrix
        .iter()
        .enumerate()
        .find_map(|(y, r)| {
            r.iter().enumerate().find_map(|(x, &c)| {
                if c == b'^' {
                    Some(Point2D::new(x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut infinite_loop = 0;

    for (x, y, v) in matrix
        .iter()
        .enumerate()
        .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, v)| (x, y, v)))
    {
        if x == starting_point.x && y == starting_point.y {
            continue;
        }
        if v == &b'#' {
            continue;
        }

        let mut point = starting_point.clone();
        let mut direction = Direction::North;
        let mut positions = HashSet::new();

        positions.insert(format!("{}.{}:{}", point.x, point.y, direction));

        loop {
            let next = point.step(direction);

            if next.x == x && next.y == y {
                direction = match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                    _ => panic!("Invalid direction"),
                };
                continue;
            }

            match matrix.at_point(&next) {
                None => {
                    break;
                }
                Some(b'#') => {
                    direction = match direction {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                        _ => panic!("Invalid direction"),
                    };
                }
                _ => {
                    point = next;
                    let is_new = positions.insert(format!("{}.{}:{}", point.x, point.y, direction));
                    if !is_new {
                        infinite_loop += 1;
                        break;
                    }
                }
            };
        }
    }

    infinite_loop
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn part_one_example() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part_one(input), 41);
    }

    #[test]
    fn part_two_example() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part_two(input), 6);
    }
}
