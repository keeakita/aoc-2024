use anyhow::{anyhow, Result};
use log::debug;
use regex::Regex;

fn slurp(day: i64) -> Result<String> {
    Ok(std::fs::read_to_string(format!("inputs/{:02}.txt", day))?)
}

fn main() -> Result<()> {
    env_logger::init();
    let input = slurp(3)?;

    let total = running_total(&input)?;
    println!("Total sum of multiplications: {}", total);

    Ok(())
}

// Part 1
fn running_total(input: &str) -> Result<i64> {
    let mut running_total = 0;
    let match_mult = Regex::new(r"mul\((\d+),(\d+)\)")?;

    for capture in match_mult.captures_iter(&input) {
        let (full, [op1, op2]) = capture.extract();
        debug!("capture: {}", full);
        let op1: i64 = op1.parse()?;
        let op2: i64 = op2.parse()?;
        debug!("mul({op1}, {op2})");
        running_total += op1 * op2;
    }

    Ok(running_total)
}
