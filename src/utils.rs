pub fn position_neighbours(
    i: usize,
    j: usize,
    i_min: usize,
    i_max: usize,
    j_min: usize,
    j_max: usize,
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
        up.and_then(|i| left.map(|j| (i, j))),
        up.map(|i| (i, j)),
        up.and_then(|i| right.map(|j| (i, j))),
        left.map(|j| (i, j)),
        right.map(|j| (i, j)),
        down.and_then(|i| left.map(|j| (i, j))),
        down.map(|i| (i, j)),
        down.and_then(|i| right.map(|j| (i, j))),
    ]
    .into_iter()
    .flatten()
}
