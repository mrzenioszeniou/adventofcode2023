use std::collections::HashSet;

#[allow(clippy::needless_range_loop)]
pub fn solve() -> anyhow::Result<()> {
    let mut map: Vec<Vec<_>> = std::fs::read_to_string("res/day10.txt")?
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut start_i = 0;
    let mut start_j = 0;
    'outer: for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                start_i = i;
                start_j = j;
                map[i][j] = '|'; // input specific
                break 'outer;
            }
        }
    }

    let mut prev_a = (start_i, start_j);
    let mut curr_a = (start_i - 1, start_j); // input specific
    let mut prev_b = (start_i, start_j);
    let mut curr_b = (start_i + 1, start_j); // input specific
    let mut part_1 = 1;
    let mut loop_tiles = HashSet::from([(start_i, start_j), curr_a, curr_b]);

    while curr_a != curr_b {
        let tmp = curr_a;
        curr_a = next(prev_a, curr_a, &map)?;
        prev_a = tmp;
        loop_tiles.insert(curr_a);

        let tmp = curr_b;
        curr_b = next(prev_b, curr_b, &map)?;
        prev_b = tmp;
        loop_tiles.insert(curr_b);

        part_1 += 1;
    }

    println!("Part 1: {part_1}\nPart 2: ??",);
    Ok(())
}

/// ```
/// o----j---->
/// |
/// i
/// |
/// ⌄
/// ```
fn next(
    (prev_i, prev_j): (usize, usize),
    (curr_i, curr_j): (usize, usize),
    map: &[Vec<char>],
) -> anyhow::Result<(usize, usize)> {
    Ok(match map[curr_i][curr_j] {
        // is a vertical pipe connecting north and south.
        '|' => {
            if prev_i + 1 == curr_i {
                (curr_i + 1, curr_j)
            } else {
                (curr_i - 1, curr_j)
            }
        }

        // is a horizontal pipe connecting east and west.
        '-' => {
            if prev_j + 1 == curr_j {
                (curr_i, curr_j + 1)
            } else {
                (curr_i, curr_j - 1)
            }
        }
        // is a 90-degree bend connecting north and east.
        'L' => {
            if prev_i + 1 == curr_i {
                (curr_i, curr_j + 1)
            } else {
                (curr_i - 1, curr_j)
            }
        }
        // is a 90-degree bend connecting north and west.
        'J' => {
            if prev_i + 1 == curr_i {
                (curr_i, curr_j - 1)
            } else {
                (curr_i - 1, curr_j)
            }
        }
        // is a 90-degree bend connecting south and west.
        '7' => {
            if curr_i + 1 == prev_i {
                (curr_i, curr_j - 1)
            } else {
                (curr_i + 1, curr_j)
            }
        }
        // is a 90-degree bend connecting south and east.
        'F' => {
            if curr_i + 1 == prev_i {
                (curr_i, curr_j + 1)
            } else {
                (curr_i + 1, curr_j)
            }
        }
        // is ground; there is no pipe in this tile.
        '.' => anyhow::bail!("this is not a pipe"),
        // is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
        'S' => anyhow::bail!("you are supposed to replace the S manually"),
        c => anyhow::bail!("unexpected character `{c}`"),
    })
}
