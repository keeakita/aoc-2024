use std::collections::HashMap;

use anyhow::Result;

fn slurp(day: i64) -> Result<String> {
    Ok(std::fs::read_to_string(format!("inputs/{:02}.txt", day))?)
}

fn main() -> Result<()> {
    let input = slurp(1)?;

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split("   ");
        // TODO: real error handling
        let left_part: i64 = parts.next().unwrap().parse()?;
        left.push(left_part);

        let right_part: i64 = parts.next().unwrap().parse()?;
        right.push(right_part);
    }

    let delta = total_delta(&left, &right)?;
    println!("Total delta: {}", delta);

    let matching = similarity_score(&left, &right)?;
    println!("Total similarity score: {}", matching);

    Ok(())
}

// Part 1
fn total_delta(left: &[i64], right: &[i64]) -> Result<i64> {
    let mut left = Vec::from(left);
    let mut right = Vec::from(right);

    // Sort backwards for efficient popping off the end of min value
    let mut total: i64 = 0;
    left.sort_by(|a, b| b.cmp(a));
    right.sort_by(|a, b| b.cmp(a));

    assert!(left.len() == right.len());

    while left.len() > 0 {
        total += (left.pop().unwrap() - right.pop().unwrap()).abs();
    }

    Ok(total)
}

// Part 2
fn similarity_score(left: &[i64], right: &[i64]) -> Result<i64> {
    let mut right_counts: HashMap<&i64, i64> = HashMap::with_capacity(right.len());
    let mut score = 0;

    for num in right {
        let count = right_counts.get(num).unwrap_or(&0) + 1;
        right_counts.insert(num, count);
    }

    for num in left {
        if right_counts.contains_key(num) {
            // TODO: Clean this up >.>
            score += num * right_counts.get(num).unwrap();
        }
    }

    Ok(score)
}
