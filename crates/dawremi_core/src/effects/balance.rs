use super::*;

pub struct Balance {
    /// Value between -1 and 1.
    /// 0 is centered, -1 is hard left pan, 1 is hard right pan
    pub balance: Automation<f64>,
}
impl Effect for Balance {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        input
            .iter()
            .enumerate()
            .map(|(idx, val)| val.balance(self.balance.value(idx)))
            .collect()
    }
}
