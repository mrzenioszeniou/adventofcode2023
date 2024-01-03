use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Context;

#[allow(clippy::while_let_on_iterator)]
pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("res/day19.txt")?;
    let mut lines = input.lines();

    let mut workflows = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let (name, rules_str) = sscanf::scanf!(line, "{}{{{}}}", String, &str)
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let rules = rules_str
            .split(',')
            .map(|r| r.parse())
            .collect::<Result<_, _>>()?;

        workflows.insert(name.clone(), Workflow { rules });
    }

    let mut parts = vec![];
    for line in lines {
        let part = serde_json::from_str::<HashMap<char, usize>>(line)?;
        parts.push(part);
    }

    let mut part_1 = 0;

    for part in parts {
        if process_part(&part, &workflows)? {
            part_1 += part.values().sum::<usize>()
        }
    }

    println!("Part 1: {part_1}\nPart 2: ??");
    Ok(())
}

fn process_part(
    part: &HashMap<char, usize>,
    workflows: &HashMap<String, Workflow>,
) -> anyhow::Result<bool> {
    let mut workflow = workflows.get("in").context("can't find 'in' workflow")?;

    loop {
        for rule in workflow.rules.iter() {
            let outcome = if let Some(condition) = rule.condition.as_ref() {
                let value = part.get(&condition.cat).context("missing category")?;
                if condition.cmp == value.cmp(&condition.val) {
                    &rule.outcome
                } else {
                    continue;
                }
            } else {
                &rule.outcome
            };

            match outcome {
                Outcome::Accept => return Ok(true),
                Outcome::Reject => return Ok(false),
                Outcome::Goto(goto) => {
                    if let Some(next) = workflows.get(goto) {
                        workflow = next;
                    } else {
                        anyhow::bail!("can't find workflow '{goto}'");
                    }
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    outcome: Outcome,
}

#[derive(Debug)]
struct Condition {
    cat: char,
    cmp: std::cmp::Ordering,
    val: usize,
}

#[derive(Debug, Clone)]
enum Outcome {
    Accept,
    Reject,
    Goto(String),
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            s => Self::Goto(s.to_string()),
        })
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            let (cat, cmp_char, val, outcome_str) =
                sscanf::scanf!(s, "{}{}{}:{}", char, char, usize, &str)
                    .map_err(|e| anyhow::Error::msg(e.to_string()))?;

            let cmp = match cmp_char {
                '<' => std::cmp::Ordering::Less,
                '>' => std::cmp::Ordering::Greater,
                c => anyhow::bail!("unexpected comparator char '{c}'"),
            };

            let outcome = Outcome::from_str(outcome_str)?;

            Ok(Self {
                condition: Some(Condition { cat, cmp, val }),
                outcome,
            })
        } else {
            Ok(Self {
                condition: None,
                outcome: Outcome::from_str(s)?,
            })
        }
    }
}
