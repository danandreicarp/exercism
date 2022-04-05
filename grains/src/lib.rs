pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s) {
        panic!("Square must be between 1 and 64");
    }

    (2_u64).pow(s - 1)
}

pub fn total() -> u64 {
    // 2^64 doesn't fit in u64, so we compute s^63, subtract 1, the multiply by 2, then add 1
    // which gives 2^64 - 1, which is what we need
    let mut almost_all = (2_u64).pow(63) - 1;

    almost_all = almost_all * 2 + 1;

    println!("total number of grains on the board: {}", almost_all);

    almost_all
}
