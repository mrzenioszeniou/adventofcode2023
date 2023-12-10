pub fn position_neighbours(
    i: usize,
    j: usize,
    i_min: usize,
    i_max: usize,
    j_min: usize,
    j_max: usize,
    diagonal: bool,
) -> impl Iterator<Item = (usize, usize)> {
    assert!(i >= i_min);
    assert!(i <= i_max);
    assert!(j >= j_min);
    assert!(j <= j_max);

    let up = i.checked_sub(1).filter(|i| *i >= i_min);
    let down = i.checked_add(1).filter(|i| *i <= i_max);
    let left = j.checked_sub(1).filter(|j| *j >= j_min);
    let right = j.checked_add(1).filter(|j| *j <= j_max);

    [
        diagonal
            .then(|| up.and_then(|i| left.map(|j| (i, j))))
            .flatten(),
        up.map(|i| (i, j)),
        diagonal
            .then(|| up.and_then(|i| right.map(|j| (i, j))))
            .flatten(),
        left.map(|j| (i, j)),
        right.map(|j| (i, j)),
        diagonal
            .then(|| down.and_then(|i| left.map(|j| (i, j))))
            .flatten(),
        down.map(|i| (i, j)),
        diagonal
            .then(|| down.and_then(|i| right.map(|j| (i, j))))
            .flatten(),
    ]
    .into_iter()
    .flatten()
}

pub fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / (gcd(a as isize, b as isize) as usize)
}

pub fn lcm_many(nums: &[usize]) -> usize {
    if nums.len() < 2 {
        panic!("Requested the least common multiple of less than 2 numbers");
    }

    if nums.len() == 2 {
        return lcm(nums[0], nums[1]);
    }

    lcm(nums[0], lcm_many(&nums[1..]))
}
