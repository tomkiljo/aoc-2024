#[macro_export]
macro_rules! create_solver {
    ($part_one:ident, $part_two:ident) => {
        pub fn solve(input: Vec<u8>, part: &Part) -> anyhow::Result<String> {
            let result = match part {
                Part::One => $part_one(input),
                Part::Two => $part_two(input),
            };
            Ok(format!("{}", result))
        }
    };
}
