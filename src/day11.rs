use std::collections::BTreeSet;

use anyhow::bail;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("res/day11.txt")?;

    let mut n = 0;
    let mut m = 0;
    let mut galaxies = BTreeSet::new();
    let mut empty_lines = BTreeSet::new();

    for (i, line) in input.lines().enumerate() {
        let mut line_is_empty = true;
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => {
                    line_is_empty = false;
                    galaxies.insert((i as isize, j as isize));
                }
                c => bail!("unknown character `{c}` in input file"),
            }
        }

        if line_is_empty {
            empty_lines.insert(i as isize);
        }

        m = m.max(line.len());
        n = n.max(i);
    }

    let mut empty_columns = (0..m as isize).collect::<BTreeSet<_>>();
    galaxies.iter().for_each(|(_, j)| {
        empty_columns.remove(j);
    });

    let mut part_1 = 0;
    let mut part_2 = 0;

    for start_galaxy in galaxies.iter() {
        for end_galaxy in galaxies.iter().skip_while(|g| start_galaxy != *g) {
            let simple_dist = (end_galaxy.0 - start_galaxy.0).unsigned_abs()
                + (end_galaxy.1 - start_galaxy.1).unsigned_abs();

            part_1 += simple_dist;
            part_2 += simple_dist;

            let min_i = start_galaxy.0.min(end_galaxy.0);
            let max_i = start_galaxy.0.max(end_galaxy.0);

            let empty_lines = empty_lines.range(min_i..max_i + 1).count();
            part_1 += empty_lines;
            part_2 += empty_lines * 999_999;

            let min_j = start_galaxy.1.min(end_galaxy.1);
            let max_j = start_galaxy.1.max(end_galaxy.1);

            let empty_columns = empty_columns.range(min_j..max_j + 1).count();
            part_1 += empty_columns;
            part_2 += empty_columns * 999_999;
        }
    }

    println!("Part 1: {part_1}\nPart 2: {part_2}");

    Ok(())
}
