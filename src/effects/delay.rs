use super::*;
use crate::helpers::delay_line::DelayLine;

pub struct Delay {
    pub delay_time: usize,
    pub feedback: Automation<f64>,
}
impl Effect for Delay {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        let mut delay_left = DelayLine::new(self.delay_time);
        let mut delay_right = DelayLine::new(self.delay_time);

        input
            .iter()
            .enumerate()
            .map(|(idx, frame)| {
                let feedback = self.feedback.value(idx);
                frame.map_left_right(
                    |sample| {
                        let ret = sample + delay_left.read();
                        delay_left.write_and_advance(ret * feedback);
                        ret
                    },
                    |sample| {
                        let ret = sample + delay_right.read();
                        delay_right.write_and_advance(ret * feedback);
                        ret
                    },
                )
            })
            .collect()
    }
}
