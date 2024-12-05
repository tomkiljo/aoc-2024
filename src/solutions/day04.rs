use crate::{create_solver, Part};

create_solver!(part_one, part_two);

fn part_one(input: Vec<u8>) -> i64 {
    let matrix: Vec<Vec<u8>> = input
        .split(|&i| match i {
            b'\n' | b'\r' => true,
            _ => false,
        })
        .map(|r| r.to_vec())
        .collect();

    let mut xmases = 0;
    for r in 0..matrix.len() {
        for c in 0..matrix[r].len() {
            if matrix[r][c] == b'X' {
                if Some(&b'M') == matrix[r].get(c + 1)
                    && Some(&b'A') == matrix[r].get(c + 2)
                    && Some(&b'S') == matrix[r].get(c + 3)
                {
                    xmases += 1;
                }

                if c >= 3
                    && Some(&b'M') == matrix[r].get(c - 1)
                    && Some(&b'A') == matrix[r].get(c - 2)
                    && Some(&b'S') == matrix[r].get(c - 3)
                {
                    xmases += 1;
                }

                if Some(&b'M') == matrix.get(r + 1).and_then(|r| r.get(c))
                    && Some(&b'A') == matrix.get(r + 2).and_then(|r| r.get(c))
                    && Some(&b'S') == matrix.get(r + 3).and_then(|r| r.get(c))
                {
                    xmases += 1;
                }

                if r >= 3
                    && Some(&b'M') == matrix.get(r - 1).and_then(|r| r.get(c))
                    && Some(&b'A') == matrix.get(r - 2).and_then(|r| r.get(c))
                    && Some(&b'S') == matrix.get(r - 3).and_then(|r| r.get(c))
                {
                    xmases += 1;
                }

                if Some(&b'M') == matrix.get(r + 1).and_then(|r| r.get(c + 1))
                    && Some(&b'A') == matrix.get(r + 2).and_then(|r| r.get(c + 2))
                    && Some(&b'S') == matrix.get(r + 3).and_then(|r| r.get(c + 3))
                {
                    xmases += 1;
                }

                if r >= 3
                    && c >= 3
                    && Some(&b'M') == matrix.get(r - 1).and_then(|r| r.get(c - 1))
                    && Some(&b'A') == matrix.get(r - 2).and_then(|r| r.get(c - 2))
                    && Some(&b'S') == matrix.get(r - 3).and_then(|r| r.get(c - 3))
                {
                    xmases += 1;
                }

                if c >= 3
                    && Some(&b'M') == matrix.get(r + 1).and_then(|r| r.get(c - 1))
                    && Some(&b'A') == matrix.get(r + 2).and_then(|r| r.get(c - 2))
                    && Some(&b'S') == matrix.get(r + 3).and_then(|r| r.get(c - 3))
                {
                    xmases += 1;
                }

                if r >= 3
                    && Some(&b'M') == matrix.get(r - 1).and_then(|r| r.get(c + 1))
                    && Some(&b'A') == matrix.get(r - 2).and_then(|r| r.get(c + 2))
                    && Some(&b'S') == matrix.get(r - 3).and_then(|r| r.get(c + 3))
                {
                    xmases += 1;
                }
            }
        }
    }

    xmases
}

fn part_two(input: Vec<u8>) -> i64 {
    let matrix: Vec<Vec<u8>> = input
        .split(|&i| match i {
            b'\n' | b'\r' => true,
            _ => false,
        })
        .map(|r| r.to_vec())
        .collect();

    let mut xmases = 0;
    for r in 0..matrix.len() {
        for c in 0..matrix[r].len() {
            if matrix[r][c] == b'A' {
                // M.S
                // .A.
                // M.S
                if r >= 1
                    && c >= 1
                    && Some(&b'M') == matrix.get(r - 1).and_then(|r| r.get(c - 1))
                    && Some(&b'S') == matrix.get(r - 1).and_then(|r| r.get(c + 1))
                    && Some(&b'M') == matrix.get(r + 1).and_then(|r| r.get(c - 1))
                    && Some(&b'S') == matrix.get(r + 1).and_then(|r| r.get(c + 1))
                {
                    xmases += 1;
                }
                // S.M
                // .A.
                // S.M
                else if r >= 1
                    && c >= 1
                    && Some(&b'S') == matrix.get(r - 1).and_then(|r| r.get(c - 1))
                    && Some(&b'M') == matrix.get(r - 1).and_then(|r| r.get(c + 1))
                    && Some(&b'S') == matrix.get(r + 1).and_then(|r| r.get(c - 1))
                    && Some(&b'M') == matrix.get(r + 1).and_then(|r| r.get(c + 1))
                {
                    xmases += 1;
                }
                // M.M
                // .A.
                // S.S
                else if r >= 1
                    && c >= 1
                    && Some(&b'M') == matrix.get(r - 1).and_then(|r| r.get(c - 1))
                    && Some(&b'M') == matrix.get(r - 1).and_then(|r| r.get(c + 1))
                    && Some(&b'S') == matrix.get(r + 1).and_then(|r| r.get(c - 1))
                    && Some(&b'S') == matrix.get(r + 1).and_then(|r| r.get(c + 1))
                {
                    xmases += 1;
                }
                // S.S
                // .A.
                // M.M
                else if r >= 1
                    && c >= 1
                    && Some(&b'S') == matrix.get(r - 1).and_then(|r| r.get(c - 1))
                    && Some(&b'S') == matrix.get(r - 1).and_then(|r| r.get(c + 1))
                    && Some(&b'M') == matrix.get(r + 1).and_then(|r| r.get(c - 1))
                    && Some(&b'M') == matrix.get(r + 1).and_then(|r| r.get(c + 1))
                {
                    xmases += 1;
                }
            }
        }
    }

    xmases
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT.as_bytes().to_vec()), 18);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT.as_bytes().to_vec()), 9);
    }
}
