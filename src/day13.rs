pub fn solve() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("res/day13.txt")?;

    let mut part_1 = 0;
    let mut part_2 = 0;
    for pattern_str in content.split("\n\n") {
        let pattern = pattern_str.lines().map(|l| l.chars().collect()).collect();
        if let Some(line) = find_reflection_line(&pattern, 0) {
            part_1 += 100 * line;
            println!("reflection between lines {} and {line}", line - 1);
        } else if let Some(column) = find_reflection_column(&pattern, 0) {
            part_1 += column;
            println!("reflection between columns {} and {column}", column - 1);
        } else {
            anyhow::bail!("no reflection line found for {pattern:?}");
        }

        if let Some(line) = find_reflection_line(&pattern, 1) {
            part_2 += 100 * line;
        } else if let Some(column) = find_reflection_column(&pattern, 1) {
            part_2 += column;
        } else {
            anyhow::bail!("no off-by-1 reflection line found for {pattern:?}");
        }
    }

    println!("Part 1: {part_1}\nPart 2: {part_2}");
    Ok(())
}

fn check_reflection_line(line: usize, pattern: &[Vec<char>]) -> usize {
    (0..line)
        .rev()
        .zip(line..pattern.len())
        .map(|(i, j)| {
            pattern[i]
                .iter()
                .zip(pattern[j].iter())
                .filter(|(i, j)| i != j)
                .count()
        })
        .sum()
}

fn find_reflection_line(pattern: &Vec<Vec<char>>, invalid_chars: usize) -> Option<usize> {
    (1..pattern.len()).find(|&line| check_reflection_line(line, pattern) == invalid_chars)
}

fn check_reflection_column(column: usize, pattern: &[Vec<char>]) -> usize {
    (0..column)
        .rev()
        .zip(column..pattern[0].len())
        .map(|(i, j)| pattern.iter().filter(|line| line[i] != line[j]).count())
        .sum()
}

fn find_reflection_column(pattern: &[Vec<char>], invalid_chars: usize) -> Option<usize> {
    (1..pattern[0].len()).find(|&column| check_reflection_column(column, pattern) == invalid_chars)
}
