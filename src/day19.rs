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

    let mut part_2 = 0;
    for (name, workflow) in workflows.iter() {
        for (idx, rule) in workflow.rules.iter().enumerate() {
            if matches!(rule.outcome, Outcome::Accept) {
                part_2 += reverse(name, idx, &workflows)?;
            }
        }
    }

    println!("Part 1: {part_1}\nPart 2: {part_2}");
    Ok(())
}

fn reverse<'a, 'b: 'a>(
    mut workflow_name: &'a str,
    mut rule_idx: usize,
    workflows: &'b HashMap<String, Workflow>,
) -> anyhow::Result<usize> {
    // print!("Reversing {workflow_name}[{rule_idx}] |");

    let mut ranges = HashMap::from([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);

    'workflows: loop {
        let workflow = workflows
            .get(workflow_name)
            .context("can't find workflow")?;

        for idx in 0..=rule_idx {
            if let Some(Condition { cat, cmp, val }) = workflow.rules[idx].condition.as_ref() {
                let range = ranges.get_mut(cat).context("no range found")?;

                match cmp {
                    std::cmp::Ordering::Less => {
                        if idx == rule_idx {
                            range.1 = range.1.min(*val - 1);
                        } else {
                            range.0 = range.0.max(*val);
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        if idx == rule_idx {
                            range.0 = range.0.max(*val + 1)
                        } else {
                            range.1 = range.1.min(*val);
                        }
                    }
                    std::cmp::Ordering::Equal => unreachable!(),
                }
            }
        }

        // if we're at the topmost workflow we're done
        if workflow_name == "in" {
            let count = ranges.values().map(|(min, max)| max - min + 1).product();
            // println!("{count} combos <- {ranges:?}");
            return Ok(count);
        }

        // otherwise we need the rule that sent us here
        for (name, workflow) in workflows.iter() {
            for i in 0..workflow.rules.len() {
                if let Outcome::Goto(ref target) = workflow.rules[i].outcome {
                    if target == workflow_name {
                        workflow_name = name.as_str();
                        rule_idx = i;
                        continue 'workflows;
                    }
                }
            }
        }

        unreachable!();
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
