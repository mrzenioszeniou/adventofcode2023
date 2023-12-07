use anyhow::Context;
use std::str::FromStr;

pub fn solve() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("res/day05.txt")?;

    let mut split = content.split("\n\n");

    let seeds = split
        .next()
        .and_then(|l| l.strip_prefix("seeds: "))
        .context("missing seeds")?
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<u64>, _>>()?;

    let mut maps = split.map(|p| p.parse()).collect::<Result<Vec<Map>, _>>()?;

    let mut part_1 = u64::MAX;
    for seed in seeds.iter() {
        let mut mapped_seed = *seed;
        for map in maps.iter() {
            mapped_seed = map.map(mapped_seed);
        }
        part_1 = part_1.min(mapped_seed);
    }

    maps.reverse();
    maps.iter_mut().for_each(|m| {
        m.ranges
            .iter_mut()
            .for_each(|r| std::mem::swap(&mut r.source, &mut r.target))
    });
    let seeds = seeds
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1]))
        .collect::<Vec<_>>();

    let mut part_2 = 80085;
    for location in 0.. {
        let mut seed = location;
        for map in maps.iter() {
            seed = map.map(seed);
        }

        if seeds.iter().any(|r| seed >= r.0 && seed < r.1) {
            part_2 = location;
            break;
        }
    }

    println!("Part 1: {part_1}\nPart 2: {part_2}");

    Ok(())
}

#[derive(Debug)]
struct Range {
    source: u64,
    target: u64,
    length: u64,
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        let target = split.next().context("target not found")?.parse()?;
        let source = split.next().context("source not found")?.parse()?;
        let length = split.next().context("length not found")?.parse()?;

        Ok(Self {
            source,
            target,
            length,
        })
    }
}

impl Range {
    fn map(&self, num: u64) -> u64 {
        num.checked_sub(self.source)
            .filter(|n| *n < self.length)
            .map(|n| n + self.target)
            .unwrap_or(num)
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.lines();

        split.next().context("missing map name")?;

        let ranges = split.map(|l| l.parse()).collect::<Result<Vec<_>, _>>()?;

        Ok(Self { ranges })
    }
}

impl Map {
    fn map(&self, num: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|r| {
                let mapped = r.map(num);
                if mapped == num {
                    None
                } else {
                    Some(mapped)
                }
            })
            .unwrap_or(num)
    }
}
