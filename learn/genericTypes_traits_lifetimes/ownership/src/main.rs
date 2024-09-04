#![allow(dead_code)]

/// Returns the n-th largest element in a slice
fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    let mut elem_refs: Vec<&T> = elems.iter().collect();
    elem_refs.sort();
    let t = elem_refs[n];
    return t.clone();
}

struct TestResult {
    /// Student's scores on a test
    scores: Vec<usize>, 

    /// A possible value to curve all scores
    curve: Option<usize>
}
impl TestResult {
    pub fn get_curve(&self) -> &Option<usize> {
        &self.curve
    }

    /// if there is a curve, then increments all scores by the curve
    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.curve {
            for score in self.scores.iter_mut() {
                *score += curve;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
