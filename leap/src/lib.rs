fn divides(n: u64, d: u64) -> bool {
    n % d == 0
}

pub fn is_leap_year(year: u64) -> bool {
    match year {
        x if divides(x, 400) => true,
        x if divides(x, 100) => false,
        x if divides(x, 4) => true,
        _ => false,
    }
}
