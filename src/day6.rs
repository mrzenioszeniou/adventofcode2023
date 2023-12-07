use anyhow::Context;

pub fn solve() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("res/day06.txt")?;
    let mut lines = content.lines();

    let times = lines
        .next()
        .context("missing times")?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, _>>()?;

    let distances = lines
        .next()
        .context("missing distances")?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, _>>()?;

    let part_1 = times
        .iter()
        .cloned()
        .zip(distances.iter().cloned())
        .map(|(time, dist)| count(time, dist))
        .product::<usize>();

    let mut lines = content.lines();

    let time = lines
        .next()
        .context("missing times")?
        .strip_prefix("Time:")
        .context("invalid times")?
        .replace(' ', "")
        .parse()?;

    let distance = lines
        .next()
        .context("missing distances")?
        .strip_prefix("Distance:")
        .context("invalid distances")?
        .replace(' ', "")
        .parse()?;

    let part_2 = count(time, distance);

    println!("Part 1: {part_1}\nPart 2: {part_2}");
    Ok(())
}

fn count(time: usize, dist: usize) -> usize {
    let mut winning_strategies = 0;
    let mut found = false;
    for hold in 1..time {
        let distance = (time - hold) * hold;

        if distance > dist {
            winning_strategies += 1;
            found = true;
        } else if found {
            break;
        }
    }

    winning_strategies
}
