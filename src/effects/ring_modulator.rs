use super::*;

pub struct RingModulator {
    /// Carrier signal for modulation
    pub carrier: Automation<f64>,
}
impl Effect for RingModulator {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        input
            .iter()
            .enumerate()
            .map(|(idx, val)| val * self.carrier.value(idx))
            .collect()
    }
}
