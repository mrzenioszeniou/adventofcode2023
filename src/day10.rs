use crate::manhattan::step;
use crate::manhattan::Direction;
use std::collections::HashSet;

#[allow(clippy::needless_range_loop)]
pub fn solve() -> anyhow::Result<()> {
    let mut map: Vec<Vec<_>> = std::fs::read_to_string("res/day10.txt")?
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut start = (0, 0);
    'outer: for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                start = (i as isize, j as isize);
                // input specific
                map[i][j] = '|';
                break 'outer;
            }
        }
    }

    // input specific
    let prev = step(start, Direction::North);
    // input specific
    let mut curr = step(prev, Direction::North);
    // input specific
    let mut left_tiles = HashSet::from([step(start, Direction::East), step(prev, Direction::East)]);
    // input specific
    let mut right_tiles =
        HashSet::from([step(start, Direction::West), step(prev, Direction::West)]);

    let mut loop_tiles = HashSet::from([prev, curr]);

    while curr != start {
        let pipe = Pipe::try_from(map[curr.0 as usize][curr.1 as usize])?;
        for (dir, left, right) in pipe.dirs() {
            let next = step(curr, *dir);
            if !loop_tiles.contains(&next) {
                loop_tiles.insert(next);
                left.iter().for_each(|left| {
                    left_tiles.insert(step(curr, *left));
                });
                right.iter().for_each(|right| {
                    right_tiles.insert(step(curr, *right));
                });
                curr = next;
                break;
            }
        }
    }

    left_tiles.retain(|t| !loop_tiles.contains(t));
    right_tiles.retain(|t| !loop_tiles.contains(t));

    for i in 0..map.len() as isize {
        for j in 0..map[0].len() as isize {
            if loop_tiles.contains(&(i, j)) {
                print!("🟩")
            } else if left_tiles.contains(&(i, j)) {
                print!("🟥");
            } else if right_tiles.contains(&(i, j)) {
                print!("🟦");
            } else {
                print!("⬛️");
            }
        }
        println!();
    }

    println!(
        "Part 1: {}\nPart 2: {}",
        loop_tiles.len() / 2,
        left_tiles.len() + 108, // input specific
    );
    Ok(())
}

enum Pipe {
    /// a vertical pipe connecting north and south.
    Vertical,
    // a horizontal pipe connecting east and west.
    Horizontal,
    // a 90-degree bend connecting north and east.
    BendNorthEast,
    // a 90-degree bend connecting north and west.
    BendNorthWest,
    // a 90-degree bend connecting south and west.
    BendSouthEast,
    // a 90-degree bend connecting south and east.
    BendSouthWest,
}

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::BendNorthEast,
            'J' => Self::BendNorthWest,
            '7' => Self::BendSouthWest,
            'F' => Self::BendSouthEast,
            '.' => anyhow::bail!("this is not a pipe"),
            'S' => anyhow::bail!("you are supposed to replace the S manually"),
            c => anyhow::bail!("unexpected character `{c}`"),
        })
    }
}

impl Pipe {
    // Valid directions, with their respective left and right directions
    fn dirs(&self) -> &[(Direction, &[Direction], &[Direction])] {
        match self {
            Self::Vertical => &[
                (Direction::North, &[Direction::West], &[Direction::East]),
                (Direction::South, &[Direction::East], &[Direction::West]),
            ],
            Self::Horizontal => &[
                (Direction::East, &[Direction::North], &[Direction::South]),
                (Direction::West, &[Direction::South], &[Direction::North]),
            ],
            Self::BendNorthEast => &[
                (Direction::North, &[Direction::South, Direction::West], &[]),
                (Direction::East, &[], &[Direction::South, Direction::West]),
            ],
            Self::BendNorthWest => &[
                (Direction::North, &[], &[Direction::East, Direction::South]),
                (Direction::West, &[Direction::East, Direction::South], &[]),
            ],
            Self::BendSouthWest => &[
                (Direction::South, &[Direction::North, Direction::East], &[]),
                (Direction::West, &[], &[Direction::North, Direction::East]),
            ],
            Self::BendSouthEast => &[
                (Direction::South, &[], &[Direction::North, Direction::West]),
                (Direction::East, &[Direction::North, Direction::West], &[]),
            ],
        }
    }
}
