pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // I wanted to remove 0 early... one could also do that on each run in the filter below
    let clean_factors: Vec<&u32> = factors.iter().filter(|&n| n > &0).collect();

    (1..limit)
        .into_iter()
        .filter(|&n| clean_factors.iter().any(|&factor| n % factor == 0))
        .sum()
}
