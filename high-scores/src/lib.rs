#[derive(Debug)]
pub struct HighScores<'a> {
    all_scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { all_scores: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.all_scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.all_scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.all_scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.all_scores.to_vec();
        // improve based on menb111 solution
        // sorted.sort();
        // sorted.reverse();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        sorted.truncate(3);
        sorted
    }
}
