#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores().last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores().iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // My solution
        let mut scores = self.scores().to_vec();
        scores.sort();

        let mut top_three = match scores.len() {
            i if i < 3 => scores,
            i => scores[i - 3..].to_vec(),
        };

        top_three.reverse();
        top_three

        // Nice community solution one
        // let mut scores = self.scores().to_vec();
        // scores.sort_unstable_by(|a, b| b.cmp(a));
        // scores.truncate(3);
        // scores

        // Nice community solution two
        // let mut scores = self.scores().to_vec();
        // scores.sort_unstable();
        // scores.into_iter().rev().take(3).collect::<Vec<u32>>()
    }
}
