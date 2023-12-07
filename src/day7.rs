use std::{cmp::Reverse, collections::HashMap, str::FromStr};

use anyhow::Context;

pub fn solve() -> anyhow::Result<()> {
    let mut hands = vec![];

    for line in std::fs::read_to_string("res/day07.txt")?.lines() {
        let hand = line.parse::<Hand>()?;
        hands.push(hand);
    }

    hands.sort();

    let part_1 = hands
        .iter()
        .enumerate()
        .map(|(ord, hand)| hand.bid * (ord + 1))
        .sum::<usize>();

    println!("Part 1: {part_1}");

    Ok(())
}

struct Hand {
    cards: Vec<Card>,
    combos: Vec<usize>,
    bid: usize,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let combos_cmp = self.combos.cmp(&other.combos);

        if !combos_cmp.is_eq() {
            return combos_cmp;
        }

        self.cards.cmp(&other.cards)
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        let cards = split
            .next()
            .context("hand not found")?
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        let bid = split.next().context("bid not found")?.parse()?;

        let combos = cards.iter().fold(HashMap::new(), |mut acc, e| {
            *acc.entry(*e).or_default() += 1;
            acc
        });

        let mut combos = combos.into_values().collect::<Vec<_>>();
        combos.sort_by_key(|k| Reverse(*k));

        Ok(Self { cards, combos, bid })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::N9,
            '8' => Self::N8,
            '7' => Self::N7,
            '6' => Self::N6,
            '5' => Self::N5,
            '4' => Self::N4,
            '3' => Self::N3,
            '2' => Self::N2,
            c => anyhow::bail!("`{c}` is not a valid card"),
        })
    }
}
