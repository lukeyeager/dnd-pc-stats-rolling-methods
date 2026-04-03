use rand::{Rng, RngExt};

pub fn roll_3d6(rng: &mut impl Rng) -> u32 {
    (0..3).map(|_| rng.random_range(1..=6)).sum()
}

pub fn roll6_3d6(rng: &mut impl Rng) -> [u32; 6] {
    std::array::from_fn(|_| roll_3d6(rng))
}

/// Roll a 6x6 grid of 3d6 values, build all 14 arrays (rows/cols/diagonals),
/// sort each descending, and return the lexicographically greatest one.
/// This models a party picking the best available stat array from the grid.
pub fn roll_grid(rng: &mut impl Rng) -> [u32; 6] {
    let grid: Vec<Vec<u32>> = (0..6)
        .map(|_| (0..6).map(|_| roll_3d6(rng)).collect())
        .collect();

    let mut arrays: Vec<Vec<u32>> = Vec::new();
    for i in 0..6 {
        arrays.push(grid[i].clone());
        arrays.push((0..6).map(|r| grid[r][i]).collect());
    }
    arrays.push((0..6).map(|i| grid[i][i]).collect());
    arrays.push((0..6).map(|i| grid[i][5 - i]).collect());

    // Sort each array descending, then pick the lexicographically greatest.
    // This prioritises: highest top stat, then highest 2nd stat as tiebreaker, etc.
    arrays
        .into_iter()
        .map(|mut a| {
            a.sort_unstable_by(|x, y| y.cmp(x));
            a
        })
        .max()
        .unwrap()
        .try_into()
        .unwrap()
}

pub const METHOD_NAMES: &[&str] = &["roll3", "grid"];

pub fn roll_method(method: &str, rng: &mut impl Rng) -> [u32; 6] {
    match method {
        "roll3" => roll6_3d6(rng),
        "grid" => roll_grid(rng),
        _ => panic!("unknown method: {}", method),
    }
}
