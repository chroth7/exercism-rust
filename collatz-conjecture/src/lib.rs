pub fn collatz(mut n: u64) -> Option<u64> {
    for count in 0.. {
        match n {
            0 => return None,
            1 => return Some(count),
            even if even % 2 == 0 => n /= 2,
            _ => n = n.checked_mul(3)?.checked_add(1)?,
        }
    }
    None // catch all (never hit due to conjecture)
}
