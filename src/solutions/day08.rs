use std::collections::{HashMap, HashSet};

use crate::{
    create_solver,
    util::{input::read_matrix, matrix::MatrixOps},
    Part,
};

create_solver!(part_one, part_two);

fn visualize(matrix: &Vec<Vec<u8>>, antinodes: &HashSet<(i32, i32)>) {
    matrix
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &c)| {
                    if antinodes.contains(&(x as i32, y as i32)) {
                        '#'
                    } else {
                        c as char
                    }
                })
                .collect::<String>()
        })
        .for_each(|row| println!("{}", row));
}

fn part_one(input: Vec<u8>) -> i64 {
    let matrix = read_matrix(&input);
    let matrix_w = matrix.width() as i32;
    let matrix_h = matrix.height() as i32;

    let mut antennas = HashMap::new();
    matrix
        .iter_points()
        .filter(|(_, value)| match value {
            Some(b'.') | None => false,
            _ => true,
        })
        .for_each(|(point, value)| {
            antennas
                .entry(value.unwrap())
                .or_insert(Vec::new())
                .push(point);
        });

    let antinodes: HashSet<(i32, i32)> = antennas
        .iter()
        .flat_map(|(_, v)| {
            let mut antinodes = Vec::new();
            for a in v {
                for b in v {
                    if a == b {
                        continue;
                    }
                    let dx = a.x as i32 - b.x as i32;
                    let dy = a.y as i32 - b.y as i32;
                    let nx = b.x as i32 + dx * 2;
                    let ny = b.y as i32 + dy * 2;
                    if nx >= 0 && nx < matrix_w && ny >= 0 && ny < matrix_h {
                        antinodes.push((nx, ny));
                    }
                }
            }
            antinodes
        })
        .collect();

    visualize(&matrix, &antinodes);
    antinodes.len() as i64
}

fn part_two(input: Vec<u8>) -> i64 {
    let matrix = read_matrix(&input);
    let matrix_w = matrix.width() as i32;
    let matrix_h = matrix.height() as i32;

    let mut antennas = HashMap::new();
    matrix
        .iter_points()
        .filter(|(_, value)| match value {
            Some(b'.') | None => false,
            _ => true,
        })
        .for_each(|(point, value)| {
            antennas
                .entry(value.unwrap())
                .or_insert(Vec::new())
                .push(point);
        });

    let antinodes: HashSet<(i32, i32)> = antennas
        .iter()
        .flat_map(|(_, v)| {
            let mut antinodes = HashSet::new();
            for a in v {
                for b in v {
                    if a == b {
                        continue;
                    }
                    antinodes.insert((a.x as i32, a.y as i32));
                    let dx = a.x as i32 - b.x as i32;
                    let dy = a.y as i32 - b.y as i32;
                    let mut mul = 2;
                    loop {
                        let nx = b.x as i32 + dx * mul;
                        let ny = b.y as i32 + dy * mul;
                        if nx >= 0 && nx < matrix_w && ny >= 0 && ny < matrix_h {
                            antinodes.insert((nx, ny));
                            mul += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
            antinodes
        })
        .collect();

    visualize(&matrix, &antinodes);
    antinodes.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_part_one() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part_one(input), 14);
    }

    #[test]
    fn test_part_two() {
        let input = INPUT.as_bytes().to_vec();
        assert_eq!(part_two(input), 34);
    }
}
