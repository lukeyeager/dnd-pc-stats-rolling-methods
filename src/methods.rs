use rand::{Rng, RngExt};

// --- Single-stat helpers ---

pub fn roll_3d6(rng: &mut impl Rng) -> u32 {
    (0..3).map(|_| rng.random_range(1..=6)).sum()
}

/// 4d6, drop the lowest die.
fn roll_4d6_drop_low(rng: &mut impl Rng) -> u32 {
    let mut dice: [u32; 4] = std::array::from_fn(|_| rng.random_range(1..=6));
    dice.sort_unstable();
    dice[1..].iter().sum()
}

/// 3d6, reroll the whole result once if it's below 8.
fn roll_3d6_reroll_under8(rng: &mut impl Rng) -> u32 {
    let x = roll_3d6(rng);
    if x < 8 { roll_3d6(rng) } else { x }
}

/// 3d6, but each die that rolls a 1 is rerolled once.
fn roll_3d6_reroll_1s(rng: &mut impl Rng) -> u32 {
    (0..3)
        .map(|_| {
            let d = rng.random_range(1..=6);
            if d == 1 { rng.random_range(1..=6) } else { d }
        })
        .sum()
}

/// 3d6, but any 1 counts as a 6.
fn roll_3d6_ones_are_sixes(rng: &mut impl Rng) -> u32 {
    (0..3)
        .map(|_| {
            let d = rng.random_range(1..=6);
            if d == 1 { 6 } else { d }
        })
        .sum()
}

// --- Full-array methods ---

pub fn roll6_3d6(rng: &mut impl Rng) -> [u32; 6] {
    std::array::from_fn(|_| roll_3d6(rng))
}

/// Standard array — fixed, no randomness.
pub fn standard_array(_rng: &mut impl Rng) -> [u32; 6] {
    [15, 14, 13, 12, 10, 8]
}

pub fn roll6_4d6_drop_low(rng: &mut impl Rng) -> [u32; 6] {
    std::array::from_fn(|_| roll_4d6_drop_low(rng))
}

pub fn roll6_3d6_reroll_under8(rng: &mut impl Rng) -> [u32; 6] {
    std::array::from_fn(|_| roll_3d6_reroll_under8(rng))
}

pub fn roll6_3d6_reroll_1s(rng: &mut impl Rng) -> [u32; 6] {
    std::array::from_fn(|_| roll_3d6_reroll_1s(rng))
}

pub fn roll6_3d6_ones_are_sixes(rng: &mut impl Rng) -> [u32; 6] {
    std::array::from_fn(|_| roll_3d6_ones_are_sixes(rng))
}

/// Roll `n` d6s, keep the top 18, then group into 6 stats of 3 dice each.
pub fn roll_many_sort<const N: usize>(rng: &mut impl Rng) -> [u32; 6] {
    let mut dice: Vec<u32> = (0..N).map(|_| rng.random_range(1..=6)).collect();
    dice.sort_unstable_by(|a, b| b.cmp(a)); // descending
    let top18 = &dice[..18];
    std::array::from_fn(|i| top18[i * 3..i * 3 + 3].iter().sum())
}

/// Each stat pair is generated from a single die roll: one stat goes up, one goes down.
/// Uses a d6, d8, and d10 to produce three balanced pairs.
pub fn roll_3up_3down(rng: &mut impl Rng) -> [u32; 6] {
    let d6 = rng.random_range(1..=6u32);
    let d8 = rng.random_range(1..=8u32);
    let d10 = rng.random_range(1..=10u32);
    [10 + d6, 15 - d6, 10 + d8, 15 - d8, 8 + d10, 17 - d10]
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

pub const METHOD_NAMES: &[&str] = &[
    "stdarr",
    "roll3",
    "roll4",
    "roll3_reroll_under8",
    "roll3_reroll_1s",
    "roll3_1s_are_6s",
    "roll18",
    "roll24",
    "3up3down",
    "6x6grid",
];

pub fn roll_method(method: &str, rng: &mut impl Rng) -> [u32; 6] {
    match method {
        "stdarr" => standard_array(rng),
        "roll3" => roll6_3d6(rng),
        "roll4" => roll6_4d6_drop_low(rng),
        "roll3_reroll_under8" => roll6_3d6_reroll_under8(rng),
        "roll3_reroll_1s" => roll6_3d6_reroll_1s(rng),
        "roll3_1s_are_6s" => roll6_3d6_ones_are_sixes(rng),
        "roll18" => roll_many_sort::<18>(rng),
        "roll24" => roll_many_sort::<24>(rng),
        "3up3down" => roll_3up_3down(rng),
        "6x6grid" => roll_grid(rng),
        _ => panic!("unknown method: {}", method),
    }
}
