use indexmap::IndexMap;

pub fn solve() -> anyhow::Result<()> {
    let mut part_1 = 0;

    let mut boxes: Vec<IndexMap<String, usize>> = vec![IndexMap::new(); 256];

    for step in std::fs::read_to_string("res/day15.txt")?.split(',') {
        part_1 += hash(step);

        let label = step
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect::<String>();
        let box_nr = hash(&label);

        if step.ends_with('-') {
            boxes[box_nr].shift_remove(&label);
        } else if let Some(lens) = step.split('=').nth(1) {
            let lens = lens.parse()?;
            boxes[box_nr].insert(label, lens);
        } else {
            anyhow::bail!("unexpected step `{step}`");
        }

        // println!("{step:6} {:?}", &boxes[0..4]);
    }

    // One plus the box number of the lens in question.
    // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    // The focal length of the lens.
    let part_2 = boxes
        .iter()
        .enumerate()
        .map(|(box_nr, lenses)| {
            lenses
                .values()
                .enumerate()
                .map(|(lens, &strength)| (box_nr + 1) * (lens + 1) * strength)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
    Ok(())
}

fn hash(string: &str) -> usize {
    let mut hash = 0;

    for c in string.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}
