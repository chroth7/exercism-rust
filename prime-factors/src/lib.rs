// brute force
pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut i = 2_u64;
    while i <= n {
        if n % i == 0 {
            factors.push(i);
            n = n / i
        } else {
            i = i + 1;
        }
    }
    factors
}
