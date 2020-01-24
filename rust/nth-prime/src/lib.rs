pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3, 5];
    let limit = (n + 1) as usize;

    while primes.len() < limit {
        let mut i = *primes.last().unwrap() + 1;

        loop {
            if is_prime(&primes, i) {
                primes.push(i);
                break;
            }
            i += 1;
        }
    }

    primes[n as usize]
}

fn is_prime(primes: &[u32], n: u32) -> bool {
    let sq = (n as f64).sqrt() as u32 + 1;

    for i in primes {
        if i > &sq {
            break;
        }

        if n % i == 0 {
            return false;
        }
    }

    true
}
