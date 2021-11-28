pub trait ProblemDefinition<const N: usize> {
    const DIMENSION: usize = N;
}

pub struct MaxwellWave {}

impl<const N: usize> ProblemDefinition<N> for MaxwellWave {
    const DIMENSION: usize = N;
}

mod tests {
    use crate::problem_def::*;

    fn test_prob_def() {
        let a = MaxwellWave {};
    }
}