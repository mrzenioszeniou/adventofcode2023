use std::collections::VecDeque;

use anyhow::Context;
use indexmap::IndexSet;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("res/day04.txt")?;

    let mut part_1 = 0;
    let mut part_2 = 0;
    let mut lookahead = VecDeque::new();
    for line in input.lines() {
        let (winners, numbers) = parse_line(line)?;
        let n_winners = numbers.iter().filter(|n| winners.contains(*n)).count();

        if n_winners > 0 {
            part_1 += 2_u32.pow(n_winners as u32 - 1);
        }

        let n_cards: u32 = 1 + lookahead.pop_front().unwrap_or_default();
        part_2 += n_cards;
        for i in 0..n_winners {
            if let Some(curr) = lookahead.get_mut(i) {
                *curr += n_cards;
            } else {
                lookahead.push_back(n_cards);
            }
        }
    }

    println!("Part 1: {part_1}\nPart 2: {part_2}");

    Ok(())
}

fn parse_line(line: &str) -> anyhow::Result<(IndexSet<u32>, IndexSet<u32>)> {
    let mut split = line
        .split(": ")
        .nth(1)
        .map(|s| s.split(" | "))
        .context("unexpected line format")?;

    let winners = split
        .next()
        .context("unexpected line format")?
        .split_whitespace()
        .map(|s| s.parse::<u32>())
        .collect::<Result<IndexSet<_>, _>>()?;

    let numbers = split
        .next()
        .context("unexpected line format")?
        .split_whitespace()
        .map(|s| s.parse::<u32>())
        .collect::<Result<IndexSet<_>, _>>()?;

    Ok((winners, numbers))
}
