use crate::manhattan::polygon_area;
use crate::manhattan::step_many;
use crate::manhattan::Direction;
use anyhow::Context;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("res/day18.txt")?;

    let mut points_1 = vec![];
    let mut curr_1 = (0, 0);
    let mut points_2 = vec![];
    let mut curr_2 = (0, 0);

    for line in input.lines() {
        let mut split = line.split_whitespace();

        let dir_1 = match split.next().context("no direction found")? {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            s => anyhow::bail!("unexpected direction '{s}' found"),
        };

        let steps_1 = split.next().context("no steps found")?.parse::<usize>()?;

        let hex = split
            .next()
            .context("no color found")?
            .strip_prefix("(#")
            .and_then(|s| s.strip_suffix(')'))
            .context("unexpected color pattern")?;

        let steps_2 = usize::from_str_radix(&hex[0..5], 16)?;
        let dir_2 = match &hex[5..] {
            "0" => Direction::East,
            "1" => Direction::South,
            "2" => Direction::West,
            "3" => Direction::North,
            s => anyhow::bail!("unexpected hex direction '{s}'"),
        };

        curr_2 = step_many(curr_2, dir_2, steps_2);
        points_2.push(curr_2);

        curr_1 = step_many(curr_1, dir_1, steps_1);
        points_1.push(curr_1);
    }

    let part_1 = polygon_area(points_1.into_iter());
    let part_2 = polygon_area(points_2.into_iter());

    println!("Part 1: {part_1}\nPart 2: {part_2}");
    Ok(())
}
