use anyhow::{anyhow, Result};

fn slurp(day: i64) -> Result<String> {
    Ok(std::fs::read_to_string(format!("inputs/{:02}.txt", day))?)
}

fn main() -> Result<()> {
    let input = slurp(2)?;
    let mut safe_count = 0;
    let mut safe_stabalized_count = 0;

    for line in input.lines() {
        let parts: Vec<i64> = line
            .split(" ")
            .map(|s| {
                s.parse::<i64>()
                    .map_err(|e| anyhow!("report contained invalid num: {}", e))
            })
            .collect::<Result<Vec<i64>>>()?;

        if is_safe(&parts, 0)? {
            safe_count += 1;
        }
        if is_safe(&parts, 1)? {
            safe_stabalized_count += 1;
        }
    }

    println!("Total safe dataset: {}", safe_count);
    println!(
        "Total safe dataset (with stabalizer): {}",
        safe_stabalized_count
    );

    Ok(())
}

// Part 1 & 2
fn is_safe(report: &[i64], tolerance: i64) -> Result<bool> {
    let mut iter = report.iter();
    let increasing: bool;
    let mut tolerance = tolerance;

    let first = iter
        .next()
        .ok_or_else(|| anyhow!("report must contain at least 2 numbers"))?;
    let second = iter
        .next()
        .ok_or_else(|| anyhow!("report must contain at least 2 numbers"))?;

    if (second - first).abs() > 3 {
        // Delta too large
        tolerance -= 1;
        if tolerance <= 0 {
            return Ok(false);
        }
    }

    if first < second {
        increasing = true;
    } else if second < first {
        increasing = false;
    } else {
        // Repeated numbers
        tolerance -= 1;
        if tolerance <= 0 {
            return Ok(false);
        }
        // fuck we have to advance the iterator
    }

    let mut last = second;
    for curr in iter {
        if (last - curr).abs() > 3 {
            tolerance -= 1;
            if tolerance <= 0 {
                return Ok(false);
            }
        }

        if increasing {
            if last >= curr {
                tolerance -= 1;
                if tolerance <= 0 {
                    return Ok(false);
                }
            }
        } else {
            if last <= curr {
                tolerance -= 1;
                if tolerance <= 0 {
                    return Ok(false);
                }
            }
        }
        last = curr;
    }

    // All conditions satisfied
    Ok(true)
}
