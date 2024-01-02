use anyhow::Context;
use colored::Colorize;
use palette::Srgb;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    str::FromStr,
};

use crate::manhattan::{neighbours, step, Direction};

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("res/day18.txt")?;

    let mut plan = HashMap::from([((0, 0), palette::named::WHITE)]);
    let mut curr = (0, 0);
    let mut min_i = 0;
    let mut min_j = 0;
    let mut max_i = 0;
    let mut max_j = 0;
    for line in input.lines() {
        let mut split = line.split_whitespace();

        let dir = match split.next().context("no direction found")? {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            s => anyhow::bail!("unexpected direction '{s}' found"),
        };

        let steps = split.next().context("no steps found")?.parse::<usize>()?;

        let color = Srgb::from_str(
            split
                .next()
                .context("no color found")?
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .context("unexpected color pattern")?,
        )?;

        for _ in 0..steps {
            curr = step(curr, dir);
            max_i = max_i.max(curr.0);
            max_j = max_j.max(curr.1);
            min_i = min_i.min(curr.0);
            min_j = min_j.min(curr.1);
            plan.insert(curr, color);
        }
    }

    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if let Some(color) = plan.get(&(i, j)) {
                let s = if i == 0 && j == 0 {
                    "@".white().on_red()
                } else {
                    "#".truecolor(color.red, color.green, color.blue)
                        .on_bright_black()
                };
                print!("{s}");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let mut part_1 = plan.len();

    let mut visited = HashSet::new();
    let mut to_visit = BTreeSet::from([(1, 1)]); // input-specific

    while let Some(pos) = to_visit.pop_last() {
        visited.insert(pos);

        if !plan.contains_key(&pos) {
            part_1 += 1;
            for (neighbour, _) in neighbours(pos) {
                if !visited.contains(&neighbour) {
                    to_visit.insert(neighbour);
                }
            }
        }
    }

    println!("Part 1: {part_1}\nPart 2: ??");
    Ok(())
}
