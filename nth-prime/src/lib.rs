// very basic sieve implementation
pub fn nth(n: u32) -> u32 {
    let mut prev_primes = vec![2_u32];
    let mut i = 0;
    let mut runner = 2;
    while i < n {
        if prev_primes.iter().any(|&p| runner % p == 0) {
            // not a prime... let's move on
            runner += 1;
        } else {
            // found a prime
            i += 1;
            prev_primes.push(runner);
        }
    }
    return runner;
}
