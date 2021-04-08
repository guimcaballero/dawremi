use super::*;

pub struct Volume {
    pub mult: Automation<f64>,
}
impl Effect for Volume {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        input
            .iter()
            .enumerate()
            .map(|(idx, val)| val * self.mult.value(idx))
            .collect()
    }
}
