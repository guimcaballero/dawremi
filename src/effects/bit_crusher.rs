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

                    let left = (((val.left + 1.) * mult).round() / mult) - 1.;
                    let right = (((val.right + 1.) * mult).round() / mult) - 1.;

                    Frame::new(left, right)
                })
                .collect(),
            BitCrusherMode::Ceiling => input
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    let mult = self.multiplier.value(idx);

                    let left = (((val.left + 1.) * mult).ceil() / mult) - 1.;
                    let right = (((val.right + 1.) * mult).ceil() / mult) - 1.;

                    Frame::new(left, right)
                })
                .collect(),
            BitCrusherMode::Floor => input
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    let mult = self.multiplier.value(idx);

                    let left = (((val.left + 1.) * mult).floor() / mult) - 1.;
                    let right = (((val.right + 1.) * mult).floor() / mult) - 1.;

                    Frame::new(left, right)
                })
                .collect(),
            BitCrusherMode::Truncate => input
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    let mult = self.multiplier.value(idx);

                    let left = (((val.left + 1.) * mult).trunc() / mult) - 1.;
                    let right = (((val.right + 1.) * mult).trunc() / mult) - 1.;

                    Frame::new(left, right)
                })
                .collect(),
        }
    }
}
