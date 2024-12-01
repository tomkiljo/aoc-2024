use std::env::args;

mod puzzles;
mod util;

fn main() {
    let day = args().nth(1).unwrap();
    let part = args().nth(2).unwrap();
    match day.as_str() {
        "1" | "01" => puzzles::day01::run(part.as_str()),
        _ => println!("Invalid day"),
    }
}
