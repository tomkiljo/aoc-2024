mod macros;
mod solutions;
mod util;

use std::{env::args, time::Instant};

use crate::util::input::read_file;
use crate::util::Part;

fn main() -> anyhow::Result<()> {
    let day: i8 = args().nth(1).unwrap().parse()?;
    let part: Part = args().nth(2).unwrap().parse()?;

    let file = format!("input/day{:02}.txt", day);
    let input = read_file(&file)?;

    let start = Instant::now();

    let result = match day {
        1 => solutions::day01::solve(input, part.clone())?,
        // 2 => solutions::day02::solve(input, part),
        // 3 => solutions::day03::solve(input, part),
        // 4 => solutions::day04::solve(input, part),
        // 5 => solutions::day05::solve(input, part),
        // 6 => solutions::day06::solve(input, part),
        // 7 => solutions::day07::solve(input, part),
        // 8 => solutions::day08::solve(input, part),
        // 9 => solutions::day09::solve(input, part),
        // 10 => solutions::day10::solve(input, part),
        // 11 => solutions::day11::solve(input, part),
        // 12 => solutions::day12::solve(input, part),
        // 13 => solutions::day13::solve(input, part),
        // 14 => solutions::day14::solve(input, part),
        // 15 => solutions::day15::solve(input, part),
        // 16 => solutions::day16::solve(input, part),
        // 17 => solutions::day17::solve(input, part),
        // 18 => solutions::day18::solve(input, part),
        // 19 => solutions::day19::solve(input, part),
        // 20 => solutions::day20::solve(input, part),
        // 21 => solutions::day21::solve(input, part),
        // 22 => solutions::day22::solve(input, part),
        // 23 => solutions::day23::solve(input, part),
        // 24 => solutions::day24::solve(input, part),
        // 25 => solutions::day25::solve(input, part),
        _ => return Err(anyhow::anyhow!("Invalid day {}", day)),
    };

    let elapsed = start.elapsed().as_millis();
    println!("Day {} Part {}: {} ({}ms)", day, part, result, elapsed);

    Ok(())
}
