use anyhow::Context;
use std::collections::HashMap;
use std::str::FromStr;

pub fn solve() -> anyhow::Result<()> {
    let text = std::fs::read_to_string("res/day02.txt")?;

    let mut part_1 = 0;
    let mut part_2 = 0;

    for (line_id, line) in text.lines().enumerate() {
        let game = parse_line(line)?;

        let possible = game.iter().all(|set| {
            set.get(&Color::Red).cloned().unwrap_or_default() <= 12
                && set.get(&Color::Green).cloned().unwrap_or_default() <= 13
                && set.get(&Color::Blue).cloned().unwrap_or_default() <= 14
        });

        if possible {
            part_1 += line_id + 1;
        }

        let mut max = HashMap::new();
        for (color, num) in game.into_iter().flat_map(|set| set.into_iter()) {
            let curr = max.entry(color).or_insert(num);
            *curr = std::cmp::max(*curr, num);
        }
        part_2 += max.into_values().product::<u32>();
    }

    println!("Part 1: {part_1}\nPart 2: {part_2}");

    Ok(())
}

fn parse_line(line: &str) -> anyhow::Result<Vec<HashMap<Color, u32>>> {
    // Game 1: 9 red, 5 blue, 6 green; 6 red, 13 blue; 2 blue, 7 green, 5 red
    let sets = line
        .split(": ")
        .nth(1)
        .context("no title found")?
        .split("; ");
    let mut parsed_sets = vec![];

    for set in sets {
        let mut parsed_set = HashMap::default();
        for cubes in set.split(", ") {
            let mut parts = cubes.split(' ');

            let cnt = parts
                .next()
                .and_then(|s| s.parse().ok())
                .context("number of cubes not found")?;
            let color = parts
                .next()
                .and_then(|s| s.parse().ok())
                .context("number of cubes not found")?;

            parsed_set.insert(color, cnt);
        }
        parsed_sets.push(parsed_set);
    }

    Ok(parsed_sets)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            other => anyhow::bail!("unknown color `{other}`"),
        })
    }
}
