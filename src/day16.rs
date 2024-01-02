use std::collections::HashMap;
use std::collections::HashSet;

use crate::manhattan::step;
use crate::manhattan::Direction;

pub fn solve() -> anyhow::Result<()> {
    let mut tiles: HashMap<(isize, isize), Tile> = HashMap::new();
    let mut n = 0;
    let mut m = 0;

    for (i, line) in std::fs::read_to_string("res/day16.txt")?
        .lines()
        .enumerate()
    {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            let tile = c.try_into()?;

            tiles.insert((i as isize, j as isize), tile);
            n = n.max(i + 1);
            m = n.max(j + 1);
        }
    }

    let n = n as isize;
    let m = m as isize;

    let part_1 = beam(0, 0, Direction::East, n, m, &tiles);

    let part_2 = (0..n)
        .flat_map(|i| {
            [
                beam(i, 0, Direction::East, n, m, &tiles),
                beam(i, m - 1, Direction::West, n, m, &tiles),
            ]
        })
        .max()
        .max(
            (0..m)
                .flat_map(|j| {
                    [
                        beam(0, j, Direction::South, n, m, &tiles),
                        beam(n - 1, j, Direction::North, n, m, &tiles),
                    ]
                })
                .max(),
        )
        .unwrap_or_default();

    println!("Part 1: {part_1}\nPart 2: {part_2}");

    Ok(())
}

enum Tile {
    /// `/`
    MirrorForward,
    /// `\`
    MirrorBackward,
    /// `|`
    SplitterVertical,
    /// `-`
    SplitterHorizontal,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '/' => Self::MirrorForward,
            '\\' => Self::MirrorBackward,
            '|' => Self::SplitterVertical,
            '-' => Self::SplitterHorizontal,
            c => anyhow::bail!("unexpected tile '{c}'"),
        })
    }
}

impl Tile {
    fn next_dir(&self, dir: Direction) -> &[Direction] {
        match (self, dir) {
            (Tile::MirrorForward, Direction::North) => &[Direction::East],
            (Tile::MirrorForward, Direction::South) => &[Direction::West],
            (Tile::MirrorForward, Direction::East) => &[Direction::North],
            (Tile::MirrorForward, Direction::West) => &[Direction::South],
            (Tile::MirrorBackward, Direction::North) => &[Direction::West],
            (Tile::MirrorBackward, Direction::South) => &[Direction::East],
            (Tile::MirrorBackward, Direction::East) => &[Direction::South],
            (Tile::MirrorBackward, Direction::West) => &[Direction::North],
            (Tile::SplitterVertical, Direction::North) => &[Direction::North],
            (Tile::SplitterVertical, Direction::South) => &[Direction::South],
            (Tile::SplitterVertical, Direction::East) => &[Direction::North, Direction::South],
            (Tile::SplitterVertical, Direction::West) => &[Direction::North, Direction::South],
            (Tile::SplitterHorizontal, Direction::North) => &[Direction::East, Direction::West],
            (Tile::SplitterHorizontal, Direction::South) => &[Direction::East, Direction::West],
            (Tile::SplitterHorizontal, Direction::East) => &[Direction::East],
            (Tile::SplitterHorizontal, Direction::West) => &[Direction::West],
        }
    }
}

fn beam(
    i: isize,
    j: isize,
    dir: Direction,
    n: isize,
    m: isize,
    tiles: &HashMap<(isize, isize), Tile>,
) -> usize {
    let mut beams = vec![(i, j, dir)];
    let mut hist = HashSet::new();

    while let Some(beam) = beams.pop() {
        hist.insert(beam);

        match tiles.get(&(beam.0, beam.1)) {
            Some(tile) => {
                for dir in tile.next_dir(beam.2) {
                    let next = step((beam.0, beam.1), *dir);

                    if next.0 >= 0
                        && next.0 < n
                        && next.1 >= 0
                        && next.1 < m
                        && !hist.contains(&(next.0, next.1, *dir))
                    {
                        beams.push((next.0, next.1, *dir));
                    }
                }
            }
            None => {
                let next = step((beam.0, beam.1), beam.2);
                if next.0 >= 0
                    && next.0 < n
                    && next.1 >= 0
                    && next.1 < m
                    && !hist.contains(&(next.0, next.1, beam.2))
                {
                    beams.push((next.0, next.1, beam.2));
                }
            }
        }
    }

    hist.iter()
        .map(|(i, j, _)| (*i, *j))
        .collect::<HashSet<_>>()
        .len()
}
