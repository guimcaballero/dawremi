use super::*;

pub struct BitCrusher {
    pub multiplier: Automation<f64>,
    pub mode: BitCrusherMode,
}

pub enum BitCrusherMode {
    Round,
    Ceiling,
    Floor,
    Truncate,
}

impl Effect for BitCrusher {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        match self.mode {
            BitCrusherMode::Round => input
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    let mult = self.multiplier.value(idx);

                    let mut left = val.left * mult;
                    let mut right = val.right * mult;

                    left = left.round() / mult;
                    right = right.round() / mult;

                    Frame::new(left, right)
                })
                .collect(),
            BitCrusherMode::Ceiling => input
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    let mult = self.multiplier.value(idx);

                    let mut left = val.left * mult;
                    let mut right = val.right * mult;

                    left = left.ceil() / mult;
                    right = right.ceil() / mult;

                    Frame::new(left, right)
                })
                .collect(),
            BitCrusherMode::Floor => input
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    let mult = self.multiplier.value(idx);

                    let mut left = val.left * mult;
                    let mut right = val.right * mult;

                    left = left.floor() / mult;
                    right = right.floor() / mult;

                    Frame::new(left, right)
                })
                .collect(),
            BitCrusherMode::Truncate => input
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    let mult = self.multiplier.value(idx);

                    let mut left = val.left * mult;
                    let mut right = val.right * mult;

                    left = left.trunc() / mult;
                    right = right.trunc() / mult;

                    Frame::new(left, right)
                })
                .collect(),
        }
    }
}
