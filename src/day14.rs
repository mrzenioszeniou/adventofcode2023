#![allow(clippy::needless_range_loop)] // hush now clippy

use std::collections::HashMap;

use anyhow::Context;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("res/day14.txt")?;

    let mut platform = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut history = HashMap::new();

    tilt_north(&mut platform)?;

    let part_1 = calculate_load(&platform)?;

    tilt_west(&mut platform)?;
    tilt_south(&mut platform)?;
    tilt_east(&mut platform)?;

    let mut completed_cycles = 1;

    while !history.contains_key(&platform) {
        history.insert(platform.clone(), completed_cycles);

        tilt_north(&mut platform)?;
        tilt_west(&mut platform)?;
        tilt_south(&mut platform)?;
        tilt_east(&mut platform)?;

        completed_cycles += 1;
    }

    let step = completed_cycles
        - history
            .get(&platform)
            .context("we should have a loop by now")?;

    while completed_cycles + step < 1_000_000_000 {
        completed_cycles += step;
    }

    for _ in 0..1_000_000_000 - completed_cycles {
        tilt_north(&mut platform)?;
        tilt_west(&mut platform)?;
        tilt_south(&mut platform)?;
        tilt_east(&mut platform)?;
    }

    let part_2 = calculate_load(&platform)?;

    println!("Part 1: {part_1}\nPart 2: {part_2}");
    Ok(())
}

fn calculate_load(platform: &[Vec<char>]) -> anyhow::Result<usize> {
    let mut load = 0;

    for i in 0..platform.len() {
        for j in 0..platform[i].len() {
            match platform[i][j] {
                'O' => load += platform.len() - i,

                '#' | '.' => {}

                c => anyhow::bail!("unexpected tile character `{c}`"),
            }
        }
    }

    Ok(load)
}

fn tilt_north(platform: &mut [Vec<char>]) -> anyhow::Result<()> {
    for j in 0..platform[0].len() {
        let mut empty_spot = 0;
        for i in 0..platform.len() {
            match platform[i][j] {
                '#' => {
                    empty_spot = i + 1;
                }

                'O' => {
                    let tmp = platform[empty_spot][j];
                    platform[empty_spot][j] = platform[i][j];
                    platform[i][j] = tmp;
                    empty_spot += 1;
                }

                '.' => {}

                c => anyhow::bail!("unexpected tile character `{c}`"),
            }
        }
    }

    Ok(())
}

fn tilt_east(platform: &mut [Vec<char>]) -> anyhow::Result<()> {
    for i in 0..platform.len() {
        let mut empty_spot = platform[i].len() - 1;
        for j in (0..platform[i].len()).rev() {
            match platform[i][j] {
                '#' => {
                    empty_spot = j.saturating_sub(1);
                }

                'O' => {
                    platform[i].swap(empty_spot, j);
                    empty_spot = empty_spot.saturating_sub(1);
                }

                '.' => {}

                c => anyhow::bail!("unexpected tile character `{c}`"),
            }
        }
    }

    Ok(())
}

fn tilt_south(platform: &mut [Vec<char>]) -> anyhow::Result<()> {
    for j in 0..platform[0].len() {
        let mut empty_spot = platform.len() - 1;
        for i in (0..platform.len()).rev() {
            match platform[i][j] {
                '#' => {
                    empty_spot = i.saturating_sub(1);
                }

                'O' => {
                    let tmp = platform[empty_spot][j];
                    platform[empty_spot][j] = platform[i][j];
                    platform[i][j] = tmp;
                    empty_spot = empty_spot.saturating_sub(1);
                }

                '.' => {}

                c => anyhow::bail!("unexpected tile character `{c}`"),
            }
        }
    }

    Ok(())
}

fn tilt_west(platform: &mut [Vec<char>]) -> anyhow::Result<()> {
    for i in 0..platform.len() {
        let mut empty_spot = 0;
        for j in 0..platform[i].len() {
            match platform[i][j] {
                '#' => {
                    empty_spot = j + 1;
                }

                'O' => {
                    platform[i].swap(empty_spot, j);
                    empty_spot += 1;
                }

                '.' => {}

                c => anyhow::bail!("unexpected tile character `{c}`"),
            }
        }
    }

    Ok(())
}
