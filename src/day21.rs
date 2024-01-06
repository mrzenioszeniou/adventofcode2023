use anyhow::Context;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::manhattan::neighbours;
use crate::manhattan::Position;

pub fn solve() -> anyhow::Result<()> {
    let mut n = 0;
    let mut m = 0;

    let input = std::fs::read_to_string("res/day21.txt")?;

    let mut rocks = HashSet::new();

    let mut curr = None;

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => {
                    rocks.insert((i as isize, j as isize));
                }
                'S' => {
                    curr = Some((i as isize, j as isize));
                }
                c => anyhow::bail!("unexpected garden plot '{c}'"),
            }
        }
        n = i + 1;
        m = m.max(line.len());
    }

    let n = n as isize;
    let m = m as isize;
    let curr = curr.context("couldn't find starting plot")?;

    let mut cache = HashMap::new();

    let plots = floodfill(curr, 64, &rocks, n, m, &mut cache);

    let part_1 = plots.len();

    println!("Part 1: {part_1}\nPart 2: ??");
    Ok(())
}

fn floodfill(
    curr: Position,
    steps: usize,
    rocks: &HashSet<Position>,
    n: isize,
    m: isize,
    cache: &mut HashMap<(Position, usize), HashSet<Position>>,
) -> HashSet<Position> {
    let mut ret = HashSet::new();

    if let Some(cached) = cache.get(&(curr, steps)) {
        return cached.clone();
    } else if steps == 0 {
        ret.insert(curr);
        return ret;
    }

    for ((i, j), _) in neighbours(curr) {
        if i >= 0 && i < n && j >= 0 && j < m && !rocks.contains(&(i, j)) {
            ret.extend(floodfill((i, j), steps - 1, rocks, n, m, cache));
        }
    }

    cache.insert((curr, steps), ret.clone());

    ret
}
