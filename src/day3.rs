use std::collections::HashMap;
use std::collections::HashSet;

use crate::utils::position_neighbours;

pub fn solve() -> anyhow::Result<()> {
    let schematic: Vec<Vec<char>> = std::fs::read_to_string("res/day03.txt")?
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut number_str = String::new();
    let mut is_part = false;
    let mut adjecent_gears = HashSet::new();
    let mut part_1 = 0;
    let mut gears_to_nums: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for i in 0..schematic.len() {
        for j in 0..schematic[i].len() {
            if schematic[i][j].is_numeric() {
                number_str.push(schematic[i][j]);
                for (i, j) in position_neighbours(
                    i,
                    j,
                    0,
                    schematic.len() - 1,
                    0,
                    schematic[i].len() - 1,
                    true,
                ) {
                    if !schematic[i][j].is_numeric() && schematic[i][j] != '.' {
                        is_part = true;
                    }

                    if schematic[i][j] == '*' {
                        adjecent_gears.insert((i, j));
                    }
                }
            } else {
                if is_part {
                    let num = number_str.parse::<u32>()?;
                    part_1 += num;

                    adjecent_gears
                        .drain()
                        .for_each(|gear| gears_to_nums.entry(gear).or_default().push(num));
                }
                number_str.clear();
                is_part = false;
            }
        }
    }

    let part_2 = gears_to_nums
        .into_values()
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums.into_iter().product::<u32>())
        .sum::<u32>();

    println!("Part 1: {part_1}\nPart 2: {part_2}");

    Ok(())
}
