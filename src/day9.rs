pub fn solve() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("res/day09.txt")?;

    let mut part_1 = 0;
    let mut part_2 = 0;
    for line in content.lines() {
        let mut series = line
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        part_1 += extrapolate(&series);

        series.reverse();
        part_2 += extrapolate(&series);
    }

    println!("Part 1: {part_1}\nPart 2: {part_2}");
    Ok(())
}

type Reading = i32;

fn extrapolate(series: &[Reading]) -> Reading {
    if series.iter().all(|r| *r == 0) {
        return 0;
    }

    let steps = series
        .windows(2)
        .map(|rs| rs[1] - rs[0])
        .collect::<Vec<_>>();

    let extrapolated = extrapolate(&steps);

    series.last().unwrap() + extrapolated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrapolation() {
        assert_eq!(extrapolate(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate(&[10, 13, 16, 21, 30, 45]), 68);
    }
}
