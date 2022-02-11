pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .into_iter()
        .filter(|&n| factors.iter().any(|&factor| factor > 0 && n % factor == 0))
        .sum()
}
