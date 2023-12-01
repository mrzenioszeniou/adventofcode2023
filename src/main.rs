use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let day = if let Some(day) = std::env::args().nth(1).and_then(|s| s.parse::<u32>().ok()) {
        day
    } else {
        println!("Usage: adventofcode2023 DAY");
        return Ok(());
    };

    match day {
        1 => println!("{}", day1()?),
        day => println!("Day {day} not unimplemented"),
    }

    Ok(())
}

fn day1() -> anyhow::Result<String> {
    let text = std::fs::read_to_string("res/day01.txt")?;

    let mut part_1 = 0;
    let mut part_2 = 0;

    for line in text.lines() {
        let first = line
            .chars()
            .find(|c| c.is_numeric())
            .context("no number found")?
            .to_digit(10)
            .context("non-numberical character")?;

        let last = line
            .chars()
            .rev()
            .find(|c| c.is_numeric())
            .context("no number found")?
            .to_digit(10)
            .context("non-numberical character")?;

        let number = first * 10 + last;

        part_1 += number;

        const STRUMBERS: [(&str, u32); 18] = [
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        let rev_line = line.chars().rev().collect::<String>();
        let mut first = None;
        let mut last = None;

        for (strumber, number) in STRUMBERS {
            if let Some(index) = line.find(strumber) {
                if first
                    .map(|(_curr_number, curr_index)| curr_index > index)
                    .unwrap_or(true)
                {
                    first = Some((number, index));
                }
            }

            let rev_strumber: String = strumber.chars().rev().collect();
            if let Some(index) = rev_line.find(&rev_strumber) {
                if last
                    .map(|(_, curr_index)| curr_index > index)
                    .unwrap_or(true)
                {
                    last = Some((number, index));
                }
            }
        }

        let first = first.context("first digit not found");

        match first {
            Ok(first) => part_2 += first.0 * 10,
            Err(err) => eprintln!("{err}"),
        }

        match last {
            Some((last, _)) => part_2 += last,
            None => eprintln!("last digit not found"),
        }
    }

    Ok(format!("Part 1: {part_1}\nPart 2: {part_2}"))
}
