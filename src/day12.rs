use anyhow::Context;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("res/day12.txt")?;

    let mut part_1 = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();

        let springs = split
            .next()
            .context("no records")?
            .chars()
            .map(TryFrom::try_from)
            .collect::<anyhow::Result<Vec<Spring>>>()?;

        let mut pattern = vec![];
        for n in split.next().context("no pattern")?.split(',') {
            let number = n.parse()?;
            if pattern.is_empty() {
                pattern.push(Pattern::Broken(number));
            } else {
                pattern.push(Pattern::Operational);
                pattern.push(Pattern::Broken(number));
            }
        }

        let mut combos = 0;

        // No prefix or suffix
        combos += combinations(&springs, &pattern);

        // Suffix
        pattern.push(Pattern::Operational);
        combos += combinations(&springs, &pattern);

        // Prefix
        pattern.pop();
        pattern.insert(0, Pattern::Operational);
        combos += combinations(&springs, &pattern);

        // Prefix & Suffix
        pattern.push(Pattern::Operational);
        combos += combinations(&springs, &pattern);

        if combos == 0 {
            anyhow::bail!("no combinations found for line {line}");
        } else {
            part_1 += combos;
        }
    }

    println!("Part 1: {part_1}\nPart 2: ??");

    Ok(())
}

fn combinations(springs: &[Spring], patterns: &[Pattern]) -> usize {
    if patterns.is_empty() && springs.is_empty() {
        return 1;
    } else if patterns.is_empty() || springs.is_empty() {
        return 0;
    }

    match patterns[0] {
        Pattern::Operational => {
            let mut combos = 0;

            for (i, spring) in springs.iter().enumerate() {
                if matches!(spring, Spring::Operational | Spring::Unknown) {
                    combos += combinations(&springs[i + 1..], &patterns[1..]);
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
                combinations(&springs[n..], &patterns[1..])
            } else {
                0
            }
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
enum Pattern {
    Operational,
    Broken(usize),
}
