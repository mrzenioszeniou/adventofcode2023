use anyhow::Context;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::manhattan::{a_star, neighbours, Direction};

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("res/day17.txt")?;
    let mut grid = HashMap::new();
    let mut n = 0;
    let mut m = 0;

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let loss = c.to_digit(10).context("invalid digit")?;

            grid.insert((i as isize, j as isize), loss as usize);
        }
        n = i as isize;
        m = m.max((line.len() - 1) as isize);
    }

    let done = |curr: &VecDeque<(isize, isize)>| curr.front() == Some(&(n, m));
    let nexts = |curr: &VecDeque<(isize, isize)>| {
        let head = curr.front().unwrap();

        let is_row = curr.len() == 4 && curr.iter().all(|p| p.0 == head.0);
        let is_col = curr.len() == 4 && curr.iter().all(|p| p.1 == head.1);

        let mut ret = HashSet::with_capacity(4);

        for (neighbour, _dir) in neighbours(*head) {
            let Some(loss) = grid.get(&neighbour) else {
                continue;
            };

            if (is_row && neighbour.0 == head.0)
                || (is_col && neighbour.1 == head.1)
                || curr.contains(&neighbour)
            {
                continue;
            }

            let mut next = curr.clone();
            next.push_front(neighbour);
            next.truncate(4);
            ret.insert((next, *loss));
        }

        ret
    };
    let heur = |curr: &VecDeque<(isize, isize)>| {
        let head = curr.front().unwrap();
        (n - head.0).unsigned_abs() + (m - head.1).unsigned_abs()
    };

    let debug = |_: &_, _: &_, _: &_| {};

    let (_, part_1) =
        a_star(VecDeque::from([(0, 0)]), done, nexts, heur, debug).context("no path found")?;

    let finished = |curr: &(isize, isize, Direction)| curr.0 == n && curr.1 == m;
    let nexts = |curr: &(isize, isize, Direction)| {
        let mut nexts = HashSet::new();

        for step_len in 4..=10 {
            'dir: for dir in [curr.2.turn_left(), curr.2.turn_right()] {
                let step = dir.forward_step();
                let mut next = (curr.0, curr.1);
                let mut total_loss = 0;
                for _ in 0..step_len {
                    next.0 += step.0;
                    next.1 += step.1;
                    let Some(loss) = grid.get(&next) else {
                        continue 'dir;
                    };
                    total_loss += *loss;
                }
                nexts.insert(((next.0, next.1, dir), total_loss));
            }
        }

        nexts
    };
    let heur = |curr: &(isize, isize, Direction)| {
        (n - curr.0).unsigned_abs() + (m - curr.1).unsigned_abs()
    };
    let debug = |_: &_, _: &_, _: &_| {};

    // initial direction is input-specific
    let (_, part_2) =
        a_star((0, 0, Direction::North), finished, nexts, heur, debug).context("no path found")?;

    println!("Part 1: {part_1}\nPart 2: {part_2}");
    Ok(())
}
