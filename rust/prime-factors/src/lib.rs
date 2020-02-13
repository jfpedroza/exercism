pub fn factors(n: u64) -> Vec<u64> {
    let mut number = n;
    let mut factor = 2;
    let mut factors = vec![];

    while number != 1 {
        if number % factor == 0 {
            number /= factor;
            factors.push(factor);
        } else {
            factor += 1;
        }
    }

    factors
}
