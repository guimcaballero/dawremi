use super::*;
use crate::helpers::resampling::stretch_frames;

pub struct Stretch {
    pub factor: f64,
}
impl Effect for Stretch {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        stretch_frames(input, self.factor)
    }
}
