use std::collections::HashMap;

use anyhow::Context;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("res/day12.txt")?;

    let mut cache = HashMap::new();

    let mut part_1 = 0;

    let mut parsed_input = vec![];

    for line in input.lines() {
        parsed_input.append(&mut parse_line(line, 1)?);
    }

    for (springs, patterns) in parsed_input.iter() {
        part_1 += combinations(springs, patterns, &mut cache);
    }

    println!("Part 1: {part_1}\nPart 2: ??");

    Ok(())
}

fn combinations<'a, 'b: 'a>(
    springs: &'b [Spring],
    patterns: &'b [Pattern],
    cache: &mut HashMap<(&'a [Spring], &'a [Pattern]), usize>,
) -> usize {
    if patterns.is_empty() && springs.is_empty() {
        return 1;
    } else if patterns.is_empty() || springs.is_empty() {
        return 0;
    } else if let Some(cached) = cache.get(&(springs, patterns)) {
        return *cached;
    }

    let ret = match patterns[0] {
        Pattern::Operational => {
            let mut combos = 0;

            for (i, spring) in springs.iter().enumerate() {
                if matches!(spring, Spring::Operational | Spring::Unknown) {
                    combos += combinations(&springs[i + 1..], &patterns[1..], cache);
                } else {
                    break;
                }
            }

            combos
        }

        Pattern::Broken(n) => {
            let count = springs
                .iter()
                .take_while(|s| matches!(s, Spring::Broken | Spring::Unknown))
                .take(n)
                .count();

            if count == n {
                combinations(&springs[n..], &patterns[1..], cache)
            } else {
                0
            }
        }
    };

    cache.insert((springs, patterns), ret);

    ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Broken,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Operational,
            '#' => Self::Broken,
            '?' => Self::Unknown,
            v => anyhow::bail!("unexpected spring character `{v}`"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pattern {
    Operational,
    Broken(usize),
}

fn parse_line(line: &str, repeat: usize) -> anyhow::Result<Vec<(Vec<Spring>, Vec<Pattern>)>> {
    let mut split = line.split_whitespace();

    // Parse
    let springs = split
        .next()
        .context("no records")?
        .chars()
        .map(TryFrom::try_from)
        .collect::<anyhow::Result<Vec<Spring>>>()?;

    let mut patterns = vec![];
    for n in split.next().context("no pattern")?.split(',') {
        let number = n.parse()?;
        if patterns.is_empty() {
            patterns.push(Pattern::Broken(number));
        } else {
            patterns.push(Pattern::Operational);
            patterns.push(Pattern::Broken(number));
        }
    }

    // Repeat
    let mut repeated_springs = Vec::with_capacity(springs.len() * (repeat + 1));
    let mut repeated_patterns = Vec::with_capacity(patterns.len() * (repeat + 1));
    for i in 0..repeat {
        if i > 0 {
            repeated_springs.push(Spring::Unknown);
            repeated_patterns.push(Pattern::Operational);
        }
        repeated_springs.extend(springs.clone());
        repeated_patterns.extend(patterns.clone());
    }

    let springs = repeated_springs;
    let mut patterns = repeated_patterns;

    let mut ret = Vec::with_capacity(4);

    // No prefix or suffix
    ret.push((springs.clone(), patterns.clone()));

    // Suffix
    patterns.push(Pattern::Operational);
    ret.push((springs.clone(), patterns.clone()));

    // Prefix
    patterns.pop();
    patterns.insert(0, Pattern::Operational);
    ret.push((springs.clone(), patterns.clone()));

    // Prefix & Suffix
    patterns.push(Pattern::Operational);
    ret.push((springs.clone(), patterns.clone()));

    Ok(ret)
}
