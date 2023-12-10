use std::{collections::HashMap, str::FromStr};

use anyhow::Context;

use crate::utils::lcm_many;

pub fn solve() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("res/day08.txt")?;

    let mut lines = content.lines();

    let directions = lines
        .next()
        .context("no directions")?
        .chars()
        .map(|c| c.try_into())
        .collect::<Result<Vec<Direction>, _>>()?;

    let nodes = lines
        .skip(1)
        .map(|l| l.parse::<Node>().map(|node| (node.name.clone(), node)))
        .collect::<Result<HashMap<_, _>, _>>()?;

    let mut curr = nodes.get("AAA").context("missing node AAA")?;
    let mut part_1 = 0;
    for dir in std::iter::repeat(directions.iter()).flat_map(|i| i.into_iter()) {
        if &curr.name == "ZZZ" {
            break;
        }

        curr = match dir {
            Direction::Left => nodes.get(&curr.left).context("couldn't find node")?,
            Direction::Right => nodes.get(&curr.right).context("couldn't find node")?,
        };
        part_1 += 1;
    }

    let mut currs = nodes
        .values()
        .filter(|n| n.name.ends_with('A'))
        .collect::<Vec<_>>();

    let mut part_2 = 0;
    let mut periods = vec![None; currs.len()];
    'outer: loop {
        for dir in directions.iter() {
            if periods.iter().all(|p| p.is_some()) {
                break 'outer;
            }

            let mut next_currs = Vec::with_capacity(currs.len());
            for (curr_idx, curr) in currs.iter().enumerate() {
                let next_curr = match dir {
                    Direction::Left => nodes.get(&curr.left).context("couldn't find node")?,
                    Direction::Right => nodes.get(&curr.right).context("couldn't find node")?,
                };

                if next_curr.name.ends_with('Z') {
                    periods[curr_idx] = Some(part_2 + 1);
                }

                next_currs.push(next_curr);
            }
            currs = next_currs;
            part_2 += 1;
        }
    }

    let periods = periods.into_iter().flatten().collect::<Vec<_>>();
    let lcm = lcm_many(&periods);

    println!("Part 1: {part_1}\nPart 2: {periods:?} => {lcm}");

    Ok(())
}

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'L' => Self::Left,
            'R' => Self::Right,
            c => anyhow::bail!("`{c}` is not a valid direction character"),
        })
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, left, right) = sscanf::sscanf!(s, "{} = ({}, {})", String, String, String)
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        Ok(Self { name, left, right })
    }
}
