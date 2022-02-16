// map is nicer, but I left fold there as an example
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![];
    }
    (0..(digits.len() - len + 1))
        // .fold(vec![], |mut acc, n| {
        // acc.push(digits[n..(n + len)].to_string());
        // acc
        // })
        .map(|n| digits[n..n + len].to_string())
        .collect()
}
