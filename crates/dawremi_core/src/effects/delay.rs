use super::*;
use crate::helpers::delay_line::DelayLine;

pub struct Delay {
    pub delay_time: usize,
    pub feedback: f64,
}
impl Effect for Delay {
    fn run(&self, input: Vec<f64>) -> Vec<f64> {
        let mut delay_line = DelayLine::new(self.delay_time);

        input
            .iter()
            .map(|sample| {
                let ret = sample + delay_line.read();
                delay_line.write_and_advance(ret * self.feedback);
                ret
            })
            .collect()
    }
}
