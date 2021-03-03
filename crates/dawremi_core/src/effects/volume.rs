use super::*;

pub struct Volume {
    pub mult: f64,
}
impl Effect for Volume {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        input.iter().map(|val| *val * self.mult).collect()
    }
}
