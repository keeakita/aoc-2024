use std::fmt::format;

use anyhow::Result;

fn slurp(day: i64) -> Result<String> {
    Ok(std::fs::read_to_string(format!("inputs/{:02}.txt", day))?)
}

fn main() -> Result<()> {
    let input = slurp(1)?;

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    let mut total: i64 = 0;

    for line in input.lines() {
        let mut parts = line.split("   ");
        // TODO: real error handling
        let left_part: i64 = parts.next().unwrap().parse()?;
        left.push(left_part);

        let right_part: i64 = parts.next().unwrap().parse()?;
        right.push(right_part);
    }

    // Sort backwards for efficient popping off the end of min value
    left.sort_by(|a, b| b.cmp(a));
    right.sort_by(|a, b| b.cmp(a));

    assert!(left.len() == right.len());

    while left.len() > 0 {
        total += (left.pop().unwrap() - right.pop().unwrap()).abs();
    }

    println!("Total delta: {}", total);

    Ok(())
}
