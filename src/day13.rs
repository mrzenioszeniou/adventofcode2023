pub fn solve() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("res/day13.txt")?;

    let mut part_1 = 0;
    for pattern_str in content.split("\n\n") {
        let pattern = pattern_str.lines().map(|l| l.chars().collect()).collect();
        if let Some(line) = find_reflection_line(&pattern) {
            part_1 += 100 * line;
            println!("reflection between lines {} and {line}", line - 1);
        } else if let Some(column) = find_reflection_column(&pattern) {
            part_1 += column;
            println!("reflection between columns {} and {column}", column - 1);
        } else {
            anyhow::bail!("no reflection line found for {pattern:?}");
        }
    }

    println!("Part 1: {part_1}\nPart 2: ??");
    Ok(())
}

fn check_reflection_line(line: usize, pattern: &[Vec<char>]) -> bool {
    (0..line)
        .rev()
        .zip(line..pattern.len())
        .all(|(i, j)| pattern[i] == pattern[j])
}

fn find_reflection_line(pattern: &Vec<Vec<char>>) -> Option<usize> {
    (1..pattern.len()).find(|&line| check_reflection_line(line, pattern))
}

fn check_reflection_column(column: usize, pattern: &[Vec<char>]) -> bool {
    let ret = (0..column)
        .rev()
        .zip(column..pattern[0].len())
        .all(|(i, j)| {
            let ret = pattern.iter().all(|line| line[i] == line[j]);
            // println!(
            //     "column {column} check {i} {}= {j}",
            //     if ret { '=' } else { '!' }
            // );
            ret
        });

    // println!("column {column} check = {ret}");

    ret
}

fn find_reflection_column(pattern: &Vec<Vec<char>>) -> Option<usize> {
    (1..pattern[0].len()).find(|&column| check_reflection_column(column, pattern))
}
