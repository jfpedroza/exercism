pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut digits = vec![];

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    let size = digits.len() as u32;
    let sum: u32 = digits.iter().map(|d| d.pow(size)).sum();

    sum == num
}
