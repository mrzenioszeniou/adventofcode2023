mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() -> anyhow::Result<()> {
    let day = if let Some(day) = std::env::args().nth(1).and_then(|s| s.parse::<u32>().ok()) {
        day
    } else {
        print_usage();
        return Ok(());
    };

    match day {
        1 => day1::solve()?,
        2 => day2::solve()?,
        3 => day3::solve()?,
        4 => day4::solve()?,
        5 => day5::solve()?,
        6 => day6::solve()?,
        7 => day7::solve()?,
        8 => day8::solve()?,
        9 => day9::solve()?,
        10 => day10::solve()?,
        day @ ..=25 => println!("Day {day} not unimplemented"),
        _ => print_usage(),
    }

    Ok(())
}

fn print_usage() {
    println!("usage: adventofcode2023 DAY");
    println!("    DAY   Advent calendar day [0..25] ");
}
