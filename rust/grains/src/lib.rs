pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    // Sum i in [0, n] of 2^i = 2^(n+1) - 1
    // 2u64.pow(64) - 1
    u64::max_value()
}
